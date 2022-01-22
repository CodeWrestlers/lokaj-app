from django.db import models
from django.contrib.auth.models import AbstractBaseUser
from django.contrib.auth.models import PermissionsMixin
from django.contrib.auth.models import BaseUserManager
from django.conf import settings
from django.db.models.deletion import DO_NOTHING
from dbapitrial.models import Users

class UserProfileManager(BaseUserManager): 

    def create_user(self, email, username, password=None):
        """Create a new user profile"""
        if not email:
            raise ValueError('User must have an email address.')

        if not username:
            raise ValueError('User must have an username.')


        email = self.normalize_email(email)
        user = self.model(email=email, username=username,)

        user.set_password(password)
        user.save(using=self._db)

        return user

    def create_superuser(self, email, name, password):
        """Create and save a new superuser with given details"""
        user = self.create_user(email, name, password)

        user.is_superuser = True
        user.is_staff = True
        user.save(using=self._db)

        return user

class User(AbstractBaseUser, PermissionsMixin):
    """Database model for users in the system"""
    email = models.EmailField(max_length=255, unique=True)
    username = models.CharField(max_length=255, unique=True)
    first_name = models.CharField(max_length=50, blank=True, null=True)
    last_name = models.CharField(max_length=50, blank=True, null=True)
    is_active = models.BooleanField(default=True)
    is_admin = models.BooleanField(default=False)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    # telegram_user_profile = models.OneToOneField(Users, on_delete=DO_NOTHING)

    objects = UserProfileManager()

    USERNAME_FIELD = 'username'
    REQUIRED_FIELDS=['email']

    def __str__(self):
        return self.username

    @property
    def is_staff(self):
        return self.is_admin
