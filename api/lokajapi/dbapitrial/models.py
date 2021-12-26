# This is an auto-generated Django model module.
# You'll have to do the following manually to clean this up:
#   * Rearrange models' order
#   * Make sure each model has one field with primary_key=True
#   * Make sure each ForeignKey and OneToOneField has `on_delete` set to the desired behavior
#   * Remove `managed = False` lines if you wish to allow Django to create, modify, and delete the table
# Feel free to rename the models, but don't rename db_table values or field names.
from django.db import models


class GarbageCollection(models.Model):
    garbage_type = models.ForeignKey('GarbageTypes', models.DO_NOTHING)
    collection_date = models.DateField()

    class Meta:
        managed = False
        db_table = 'garbage_collection'


class GarbageTypes(models.Model):
    name = models.CharField(max_length=-1)
    emoji = models.CharField(max_length=2)
    language_code = models.CharField(max_length=-1)

    class Meta:
        managed = False
        db_table = 'garbage_types'


class Locations(models.Model):
    city = models.CharField(max_length=1)
    country = models.CharField(max_length=1)

    class Meta:
        managed = False
        db_table = 'locations'


class Messages(models.Model):
    id = models.BigAutoField(primary_key=True)
    user_id = models.BigAutoField()
    text = models.TextField()
    utc_timestamp = models.DateTimeField()
    unix_timestamp = models.IntegerField()

    class Meta:
        managed = False
        db_table = 'messages'


class Users(models.Model):
    id = models.BigAutoField(primary_key=True)
    is_bot = models.BooleanField()
    first_name = models.CharField(max_length=-1)
    last_name = models.CharField(max_length=-1, blank=True, null=True)
    username = models.CharField(max_length=-1, blank=True, null=True)
    language_code = models.CharField(max_length=-1, blank=True, null=True)
    is_subscribed = models.BooleanField()
    utc_created = models.DateTimeField()

    class Meta:
        managed = False
        db_table = 'users'
