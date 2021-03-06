# -*- coding: utf-8 -*-
# Generated by Django 1.11.23 on 2020-11-16 21:25
from __future__ import unicode_literals

from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [
        ("chroma_core", "0029_remove_managedtarget_fields"),
    ]

    operations = [
        migrations.RemoveField(
            model_name="configuretargetjob",
            name="job_ptr",
        ),
        migrations.RemoveField(
            model_name="configuretargetjob",
            name="target",
        ),
        migrations.RemoveField(
            model_name="failbacktargetjob",
            name="job_ptr",
        ),
        migrations.RemoveField(
            model_name="failbacktargetjob",
            name="target",
        ),
        migrations.RemoveField(
            model_name="formattargetjob",
            name="job_ptr",
        ),
        migrations.RemoveField(
            model_name="formattargetjob",
            name="target",
        ),
        migrations.RemoveField(
            model_name="registertargetjob",
            name="job_ptr",
        ),
        migrations.RemoveField(
            model_name="registertargetjob",
            name="target",
        ),
        migrations.RemoveField(
            model_name="removeconfiguredtargetjob",
            name="job_ptr",
        ),
        migrations.RemoveField(
            model_name="removeconfiguredtargetjob",
            name="target",
        ),
        migrations.RemoveField(
            model_name="removetargetjob",
            name="job_ptr",
        ),
        migrations.RemoveField(
            model_name="removetargetjob",
            name="target",
        ),
        migrations.RemoveField(
            model_name="removefilesystemjob",
            name="filesystem",
        ),
        migrations.RemoveField(
            model_name="removefilesystemjob",
            name="job_ptr",
        ),
        migrations.RemoveField(
            model_name="updatenidsjob",
            name="job_ptr",
        ),
        migrations.RemoveField(
            model_name="updatedevicesjob",
            name="job_ptr",
        ),
        migrations.DeleteModel(
            name="TargetFailoverAlert",
        ),
        migrations.DeleteModel(
            name="ConfigureTargetJob",
        ),
        migrations.DeleteModel(
            name="FailbackTargetJob",
        ),
        migrations.DeleteModel(
            name="FormatTargetJob",
        ),
        migrations.DeleteModel(
            name="RegisterTargetJob",
        ),
        migrations.DeleteModel(
            name="RemoveConfiguredTargetJob",
        ),
        migrations.DeleteModel(
            name="RemoveTargetJob",
        ),
        migrations.DeleteModel(
            name="RemoveFilesystemJob",
        ),
        migrations.DeleteModel(
            name="UpdateNidsJob",
        ),
        migrations.DeleteModel(
            name="UpdateDevicesJob",
        ),
    ]
