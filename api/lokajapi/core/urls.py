
from django.urls import path

from core.api.views import CreateUserView
from core.api.views import CreateTokenView

app_name = 'user'

urlpatterns = [
    path('register/', CreateUserView.as_view(), name='register'),
    path('token/', CreateTokenView.as_view(), name='token'),
]