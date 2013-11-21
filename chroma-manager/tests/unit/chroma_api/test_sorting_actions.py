import logging
from chroma_api.utils import StatefulModelResource
from chroma_core.models import ConfigureLNetJob

from tests.unit.chroma_api.chroma_api_test_case import ChromaApiTestCase
from tests.unit.chroma_core.helper import synthetic_host

log = logging.getLogger(__name__)


class TestSortingActions(ChromaApiTestCase):
    """Test that system will sort and group all jobs and jobs via transitions

    This class mocks out job class computations, and job scheduler code:

    The emphesis is on making sure sorting and grouping works given that the
    integrated systems provide the correct information.

        JobSchedulerClient.available_jobs
        StatefulModelResource._add_verb
        JobSchedulerClient.available_transitions (indirectly, via _add_verb)

    The assumption here is that the production code, that is mocked here,
    will annotate the order and group values appropriately on the jobs
    returned.  Provided that happens, this code verifies the jobs will be sorted.
    """

    def _mock_available_jobs(self, host, expected_jobs):
        """Mock the available_jobs to return expected jobs for this host

        expected_jobs is controlled passing a list of tuples (verb, order, group)

        The JobScheduler wraps the jobs found in a dictionary.  This
        test does that wrapping in the process of this mock.
        """

        def _mock_job_dict(verb, order, group):
            return {
                'verb': verb,
                'long_description': None,
                'display_group': group,
                'display_order': order,
                'confirmation': None,
                'class_name': None,
                'args': None}

        wrapped_jobs = [_mock_job_dict(verb, order, group)
                for verb, order, group in expected_jobs]

        @classmethod
        def _get_jobs(cls, obj_list):
            #  Return these jobs for this host only.
            return {str(host.id): wrapped_jobs}

        # NB: the superclass will tear down this monkey patch
        from chroma_core.services.job_scheduler import job_scheduler_client
        self.old_available_jobs = job_scheduler_client.JobSchedulerClient.available_jobs
        job_scheduler_client.JobSchedulerClient.available_jobs = _get_jobs

    def _mock_add_verb(self, expected_jobs):
        """Mock the StatefulModelResource._add_verb method"""

        def _mock_trans_to_job_dict(verb, order, group):
            return {
                'verb': verb,
                'long_description': None,
                'display_group': group,
                'display_order': order,
                "state": None}

        wrapped_jobs = [_mock_trans_to_job_dict(verb, order, group)
                        for verb, order, group in expected_jobs]

        def _add_verb(self, stateful_object, raw_transitions):
            #  Return these jobs for this host only.
            return wrapped_jobs

        # NB: the superclass will tear down this monkey patch
        if not hasattr(self, 'add_verb'):
            from chroma_api import utils
            self.add_verb = utils.StatefulModelResource._add_verb
            utils.StatefulModelResource._add_verb = _add_verb

    def tearDown(self):

        from chroma_core.services.job_scheduler import job_scheduler_client
        job_scheduler_client.JobSchedulerClient.available_jobs = self.old_available_jobs

        from chroma_api import utils
        if hasattr(self, 'add_verb'):
            utils.StatefulModelResource._add_verb = self.add_verb

    def test_sorting_actions(self):
        """Ensure direct jobs or transition jobs are sorted together."""

        host = synthetic_host()

        # These are the values the JobScheduler would return in scrambled order
        # the job.verb, job.order and job.group fields are stubbed
        self._mock_add_verb([('Job 3', 3, 2), ('Job 1', 1, 1), ('Job 6', 6, 3)])
        self._mock_available_jobs(host, [('Job 5', 5, 3), ('Job 2', 2, 1), ('Job 4', 4, 2)])

        response = self.api_client.get("/api/host/%s/" % host.id)

        self.assertHttpOK(response)
        host = self.deserialize(response)

        received_verbs_order = [t['verb'] for t in host['available_actions']]
        expected_verbs_order = ['Job 1', 'Job 2', 'Job 3', 'Job 4', 'Job 5', 'Job 6']

        self.assertEqual(received_verbs_order, expected_verbs_order)

        received_verbs_group = [t['display_group'] for t in host['available_actions']]
        expected_verbs_group = [1, 1, 2, 2, 3, 3]

        self.assertEqual(received_verbs_group, expected_verbs_group)

    def test_add_verb(self):
        """Test that add verb turns the jobs into the correct dictionary"""

        host = synthetic_host()

        def _mock_get_job_class(begin_state, end_state, last_job_in_route=False):
            return ConfigureLNetJob  # a StateChangeJob
        host.get_job_class = _mock_get_job_class

        self.assertTrue(host.get_job_class(host.state, 'ignored') == ConfigureLNetJob)
        self.assertTrue(hasattr(host.get_job_class(host.state, 'ignored'), 'state_verb'))

        # NB: JobScheduler._fetch_jobs takes an object, but could take a class
        jobs = StatefulModelResource()._add_verb(host, ['ignored'])

        job_dict = jobs[0]
        self.assertTrue('verb' in job_dict)
        self.assertTrue('display_group' in job_dict)
        self.assertTrue('display_order' in job_dict)
        self.assertTrue('state' in job_dict)
        self.assertTrue('long_description' in job_dict)
