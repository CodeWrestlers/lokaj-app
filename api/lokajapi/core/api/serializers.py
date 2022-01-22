from django.contrib.auth import get_user_model
from django.core.exceptions import ValidationError
from rest_framework import response, serializers
from django.contrib.auth import authenticate
from django.utils.translation import ugettext_lazy as _

User = get_user_model()

class RegistrationSerializer(serializers.ModelSerializer):

    class Meta:
        model = User
        fields = ('email', 'username', 'password',)

        extra_kwargs = {
            'password': {
                'write_only': True,
                'style': {'input_type': 'password'}
            }
        }

    def validate_email(self, email):
        email = email.lower()
        if User.objects.filter(email=email):
            raise serializers.ValidationError("The e-mail is already used. Please try again with other e-mail address.")
        return email

    def validate_username(self, username):
        if User.objects.filter(username=username):
            raise serializers.ValidationError("The username is already used. Please try again with other username.")
        return username

    def validate_password(self, password):
        if not password:
            raise serializers.ValidationError("The password field cannot be blank.")
        if len(password) < 8:
            raise serializers.ValidationError("The password must be at least 8 chars long.")
        return password

    def create(self, validated_data): 
        return get_user_model().objects.create_user(**validated_data)


class LoginSerializer(serializers.Serializer):
    username = serializers.CharField()
    password = serializers.CharField(
        style={'input_type': 'password'},
        trim_whitespace=False
    )

    def validate(self, data):

        user = authenticate(
            request=self.context.get('request'),
            username=data['username'],
            password=data['password']
        )
        if not user:
            raise serializers.ValidationError('Authentication failed with provided credentials.')

        data['user'] = user
        return data



# class AuthTokenSerializer(serializers.Serializer):
#     """Serializer for the user authentication object"""
#     email = serializers.CharField()
#     password = serializers.CharField(
#         style={'input_type': 'password'},
#         trim_whitespace=False
#     )

#     def validate(self, attrs):
#         """Validate and authenticate the user"""
#         email = attrs.get('email')
#         password = attrs.get('password')

#         user = authenticate(
#             request=self.context.get('request'),
#             username=email,
#             password=password
#         )
#         if not user:
#             msg = _('Unable to authenticate with provided credentials')
#             raise serializers.ValidationError(msg, code='authentication')

#         attrs['user'] = user
#         return attrs
 