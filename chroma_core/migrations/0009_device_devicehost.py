# -*- coding: utf-8 -*-
# Generated by Django 1.11.23 on 2019-09-16 14:40
from __future__ import unicode_literals

import django.contrib.postgres.fields
from django.db import migrations, models


class Migration(migrations.Migration):

    dependencies = [("chroma_core", "0008_ostpool_json_notify")]

    operations = [
        migrations.CreateModel(
            name="Device",
            fields=[
                (
                    "id",
                    models.CharField(
                        help_text=b"Unique identifer per-device", max_length=255, primary_key=True, serialize=False
                    ),
                ),
                ("size", models.CharField(help_text=b"The size of the device in bytes", max_length=64)),
                ("usable_for_lustre", models.BooleanField(help_text=b"Is this storage device usable for Lustre")),
                ("device_type", models.CharField(help_text=b"The type of block or virtual device", max_length=64)),
                (
                    "parents",
                    django.contrib.postgres.fields.ArrayField(
                        base_field=models.CharField(max_length=255), help_text=b"", size=None
                    ),
                ),
                (
                    "children",
                    django.contrib.postgres.fields.ArrayField(
                        base_field=models.CharField(max_length=255), help_text=b"", size=None
                    ),
                ),
                ("max_depth", models.IntegerField(default=0, help_text=b"")),
            ],
        ),
        migrations.CreateModel(
            name="DeviceHost",
            fields=[
                ("id", models.AutoField(auto_created=True, primary_key=True, serialize=False, verbose_name="ID")),
                ("device_id", models.CharField(help_text=b"Unique identifer per-device", max_length=255)),
                ("fqdn", models.CharField(help_text=b"The size of the device in bytes", max_length=255)),
                ("local", models.BooleanField(help_text=b"Is this storage device usable for Lustre")),
                (
                    "paths",
                    django.contrib.postgres.fields.ArrayField(
                        base_field=models.CharField(max_length=255), help_text=b"", size=None
                    ),
                ),
                ("mount_path", models.CharField(help_text=b"", max_length=255, null=True)),
                ("fs_type", models.CharField(help_text=b"", max_length=255, null=True)),
                ("fs_label", models.CharField(help_text=b"", max_length=255, null=True)),
                ("fs_uuid", models.CharField(help_text=b"", max_length=255, null=True)),
            ],
        ),
    ]
