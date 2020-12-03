# -*- coding: utf-8 -*-
# Generated by Django 1.11.23 on 2020-10-20 11:48

from __future__ import unicode_literals

from django.db import migrations, models
from tastypie.serializers import Serializer

from chroma_core.models.step_result import StepResult
from chroma_core.models.jobs import Job


def apply_job_changes(apps, schema_editor):
    module = __import__("chroma_core.models")
    job_klass = getattr(module.models, "Job")
    for job in job_klass.objects.all():  # type: Job
        # if we do concrete_job = apps.get_model(job.content_type.app_label, job.content_type.model)
        # then concrete_job doesn't contain `description` method, so the `downcast` required
        concrete_job = job.downcast()
        job.class_name = concrete_job.__class__.__name__
        try:
            job.description_out = job.description()
        except NotImplementedError:
            job.description_out = ""
        job.cancellable_out = concrete_job.cancellable
        job.save()

    step_result_klass = getattr(module.models, "StepResult")
    serializer = Serializer()
    for sr in step_result_klass.objects.all():  # type: StepResult
        sr.class_name = sr.step_klass.__name__
        sr.args_json = serializer.to_json(sr.args)
        sr.description = sr.describe()
        sr.save()


def reverse_job_changes(_apps, _schema_editor):
    pass


class Migration(migrations.Migration):
    dependencies = [
        ("chroma_core", "0032_forgetlustreclientjob"),
    ]

    operations = [
        migrations.AddField(
            model_name="job",
            name="class_name",
            field=models.TextField(default=b""),
        ),
        migrations.AddField(
            model_name="job",
            name="description_out",
            field=models.TextField(default=b""),
        ),
        migrations.AddField(
            model_name="job",
            name="cancellable_out",
            field=models.BooleanField(default=True),
        ),
        migrations.AddField(
            model_name="stepresult",
            name="class_name",
            field=models.CharField(default=b"", max_length=128),
        ),
        migrations.AddField(
            model_name="stepresult",
            name="args_json",
            field=models.TextField(default=b"{}"),
        ),
        migrations.AddField(
            model_name="stepresult",
            name="description",
            field=models.TextField(default=b""),
        ),
        migrations.RunPython(apply_job_changes, reverse_job_changes),
    ]