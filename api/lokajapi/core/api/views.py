from rest_framework import generics, authentication, permissions
from core.api.serializers import RegistrationSerializer, LoginSerializer
from rest_framework.permissions import AllowAny
from rest_framework.authtoken.views import ObtainAuthToken
from rest_framework.settings import api_settings


class CreateUserView(generics.CreateAPIView):
    permission_classes=[AllowAny,]
    serializer_class = RegistrationSerializer


class CreateTokenView(ObtainAuthToken):
    serializer_class = LoginSerializer
    renderer_classes = api_settings.DEFAULT_RENDERER_CLASSES


# class ManageUserView(generics.RetrieveUpdateAPIView):
#     serializer_class = UserSerializer
#     authentication_classes = (authentication.TokenAuthentication,)
#     permission_classes = (permissions.IsAuthenticated,)

#     def get_object(self):
#         return self.request.user