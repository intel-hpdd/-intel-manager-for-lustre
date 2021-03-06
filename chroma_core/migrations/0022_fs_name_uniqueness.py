# -*- coding: utf-8 -*-
# Generated by Django 1.11.23 on 2019-09-07 02:19
from __future__ import unicode_literals
from django.db import migrations


forward = """
ALTER TABLE chroma_core_managedfilesystem ADD CONSTRAINT unique_name EXCLUDE USING gist (name WITH =, int4(not_deleted) WITH =) WHERE (int4(not_deleted) = 1);
"""

backward = """
ALTER TABLE chroma_core_managedfilesystem DROP CONSTRAINT unique_name;
"""


class Migration(migrations.Migration):

    dependencies = [("chroma_core", "0021_many_mountpoint")]

    operations = [migrations.RunSQL(sql=forward, reverse_sql=backward)]
