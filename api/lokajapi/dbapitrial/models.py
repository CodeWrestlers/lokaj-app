from django.core.exceptions import ValidationError
from django.db import models
from django.db.models.deletion import CASCADE
from langcodes import Language
from unixtimestampfield.fields import UnixTimeStampField


class GarbageCollection(models.Model):
    garbage_type = models.ForeignKey('GarbageTypes', on_delete=models.DO_NOTHING)
    collection_date = models.DateField(auto_now=True)

    class Meta:
        managed = False
        db_table = 'garbage_collection'


class GarbageTypes(models.Model):
    name = models.CharField(max_length=30)
    emoji = models.CharField(max_length=2)
    language_code = models.CharField(max_length=3)

    class Meta:
        managed = False
        db_table = 'garbage_types'

    def clean(self):
        self.language_code = self.language_code.lower()
        if not Language.get(self.language_code).is_valid():
            raise ValidationError('Language code is not valid.')


class Messages(models.Model):
    user = models.ForeignKey('GarbageTypes', on_delete=CASCADE)
    text = models.TextField()
    utc_timestamp = models.DateTimeField(auto_now=True)
    unix_timestamp = UnixTimeStampField(auto_now=True)

    class Meta:
        managed = False
        db_table = 'messages'


class Users(models.Model):
    is_bot = models.BooleanField()
    first_name = models.CharField(max_length=30)
    last_name = models.CharField(max_length=50, blank=True, null=True)
    username = models.CharField(max_length=30, blank=True, null=True)
    language_code = models.CharField(max_length=3, blank=True, null=True)
    is_subscribed = models.BooleanField(default=False)
    utc_created = models.DateTimeField(auto_now_add=True)

    class Meta:
        managed = False
        db_table = 'users'

    def clean(self):
        self.language_code = self.language_code.lower()
        if not Language.get(self.language_code).is_valid():
            raise ValidationError('Language code is not valid.')
