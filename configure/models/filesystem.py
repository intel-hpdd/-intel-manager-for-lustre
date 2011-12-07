
# ==============================
# Copyright 2011 Whamcloud, Inc.
# ==============================

from django.db import models
from configure.lib.job import StateChangeJob, DependOn, DependAll, Step
from configure.models.jobs import StatefulObject, Job
from monitor.models import DeletableDowncastableMetaclass, MeasuredEntity


class ManagedFilesystem(StatefulObject, MeasuredEntity):
    __metaclass__ = DeletableDowncastableMetaclass

    name = models.CharField(max_length=8)
    mgs = models.ForeignKey('ManagedMgs')

    states = ['unavailable', 'stopped', 'available', 'removed']
    initial_state = 'unavailable'

    def human_name(self):
        return self.name

    def get_available_states(self, begin_state):
        available_states = super(ManagedFilesystem, self).get_available_states(begin_state)
        # Exclude 'stopped' if we are in 'unavailable' and everything is stopped
        target_states = set([t.state for t in self.get_filesystem_targets()])
        if begin_state == 'unavailable' and not 'mounted' in target_states:
            available_states = list(set(available_states) - set(['stopped']))

        return available_states

    class Meta:
        unique_together = ('name', 'mgs')

    def get_targets(self):
        return [self.mgs.downcast()] + self.get_filesystem_targets()

    def get_filesystem_targets(self):
        from configure.models import ManagedOst, ManagedMdt
        osts = list(ManagedOst.objects.filter(filesystem = self).all())
        # NB using __str__ instead of name because name may not
        # be set in all cases
        osts.sort(lambda i, j: cmp(i.__str__()[-4:], j.__str__()[-4:]))

        return list(ManagedMdt.objects.filter(filesystem = self).all()) + osts

    def get_servers(self):
        from collections import defaultdict
        targets = self.get_targets()
        servers = defaultdict(list)
        for t in targets:
            for tm in t.managedtargetmount_set.all():
                servers[tm.host].append(tm)

        # NB converting to dict because django templates don't place nice with defaultdict
        # (http://stackoverflow.com/questions/4764110/django-template-cant-loop-defaultdict)
        return dict(servers)

    def status_string(self, target_statuses = None):
        if target_statuses == None:
            target_statuses = dict([(t, t.status_string()) for t in self.get_targets()])

        from configure.models.target import ManagedMgs
        filesystem_targets_statuses = [v for k, v in target_statuses.items() if not k.__class__ == ManagedMgs]
        all_statuses = target_statuses.values()

        good_status = set(["STARTED", "FAILOVER"])
        # If all my targets are down, I'm red, even if my MGS is up
        if not good_status & set(filesystem_targets_statuses):
            return "OFFLINE"

        # If all my targets are up including the MGS, then I'm green
        if set(all_statuses) <= set(["STARTED"]):
            return "OK"

        # Else I'm orange
        return "WARNING"

    def mgs_spec(self):
        """Return a string which is foo in <foo>:/lustre for client mounts"""
        mgs = self.mgs
        nid_specs = []
        for target_mount in mgs.managedtargetmount_set.all():
            host = target_mount.host
            nids = ",".join(host.lnetconfiguration.get_nids())
            if nids == "":
                raise ValueError("NIDs for MGS host %s not known" % host)

            nid_specs.append(nids)

        return ":".join(nid_specs)

    def mount_example(self):
        try:
            return "mount -t lustre %s:/%s /mnt/client" % (self.mgs_spec(), self.name)
        except ValueError, e:
            return "Not ready to mount: %s" % e

    def __str__(self):
        return self.name

    class Meta:
        app_label = 'configure'

    def get_conf_params(self):
        from itertools import chain
        from configure.models.conf_param import ConfParam
        params = chain(self.filesystemclientconfparam_set.all(), self.filesystemglobalconfparam_set.all())
        return ConfParam.get_latest_params(params)

    def get_deps(self, state = None):
        if not state:
            state = self.state

        deps = []

        mgs = self.mgs.downcast()
        if state != 'removed':
            deps.append(DependOn(mgs,
                    'unmounted',
                    acceptable_states = mgs.not_state('removed'),
                    fix_state = 'removed'))

        if state == 'available':
            for t in self.get_targets():
                deps.append(DependOn(t,
                    'mounted',
                    fix_state = 'unavailable'))
        elif state == 'stopped':
            for t in self.get_targets():
                deps.append(DependOn(t,
                    'unmounted',
                    fix_state = 'unavailable'))

        return DependAll(deps)

    @classmethod
    def filter_by_target(self, target):
        from configure.models import ManagedMgs
        target = target.downcast()
        if isinstance(target, ManagedMgs):
            return ManagedFilesystem.objects.filter(mgs = target)
        else:
            return [target.filesystem]

    reverse_deps = {
            'ManagedTarget': lambda mt: ManagedFilesystem.filter_by_target(mt)
            }


class DeleteFilesystemStep(Step):
    idempotent = True

    def run(self, kwargs):
        from configure.models import ManagedFilesystem
        ManagedFilesystem.delete(kwargs['filesystem_id'])


class RemoveFilesystemJob(Job, StateChangeJob):
    state_transition = (ManagedFilesystem, 'stopped', 'removed')
    stateful_object = 'filesystem'
    state_verb = "Remove"
    filesystem = models.ForeignKey('ManagedFilesystem')

    class Meta:
        app_label = 'configure'

    def description(self):
        return "Removing filesystem %s from configuration" % (self.filesystem.name)

    def get_steps(self):
        return [(DeleteFilesystemStep, {'filesystem_id': self.filesystem.id})]


class FilesystemJob():
    stateful_object = 'filesystem'

    class Meta:
        app_label = 'configure'

    def get_steps(self):
        from configure.lib.job import NullStep
        return [(NullStep, {})]


class StartStoppedFilesystemJob(FilesystemJob, Job, StateChangeJob):
    state_verb = "Start"
    state_transition = (ManagedFilesystem, 'stopped', 'available')
    filesystem = models.ForeignKey('ManagedFilesystem')

    def description(self):
        return "Start filesystem %s" % (self.filesystem.name)


class StartUnavailableFilesystemJob(FilesystemJob, Job, StateChangeJob):
    state_verb = "Start"
    state_transition = (ManagedFilesystem, 'unavailable', 'available')
    filesystem = models.ForeignKey('ManagedFilesystem')

    def description(self):
        return "Start filesystem %s" % (self.filesystem.name)


class StopUnavailableFilesystemJob(FilesystemJob, Job, StateChangeJob):
    state_verb = "Stop"
    state_transition = (ManagedFilesystem, 'unavailable', 'stopped')
    filesystem = models.ForeignKey('ManagedFilesystem')

    def description(self):
        return "Stop filesystem %s" % (self.filesystem.name)


class StopAvailableFilesystemJob(FilesystemJob, Job, StateChangeJob):
    state_verb = "Stop"
    state_transition = (ManagedFilesystem, 'available', 'stopped')
    filesystem = models.ForeignKey('ManagedFilesystem')

    def description(self):
        return "Stop filesystem %s" % (self.filesystem.name)
