from django.db import models

# Create your models here.


class Url(models.Model):
    hash = models.CharField(unique=True, max_length=15)
    alias = models.CharField(max_length=64, null=True, blank=True)
    original_url = models.CharField(max_length=4096)
    expired_on = models.DateTimeField(auto_now_add=True)
    created_on = models.DateTimeField(auto_now_add=True)
