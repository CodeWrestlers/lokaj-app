from django.test import TestCase
from django.contrib.auth import get_user_model
from django.urls import reverse
from rest_framework.test import APIClient
from rest_framework import status


register_url = reverse('user:register')
token_url = reverse('user:token')


def create_user(**kwargs):
    return get_user_model().objects.create_user(**kwargs)


class UserRegistrationTests(TestCase):

    def setUp(self):
        self.client = APIClient()
        self.payload = {
            'email': 'proztocki@lokajbot.com',
            'password': 'LewisHamilton44',
            'username': 'PRoztocki',
        }

    def test_create_valid_user_success(self):
        res = self.client.post(register_url, self.payload)
        self.assertEqual(res.status_code, status.HTTP_201_CREATED)
        user = get_user_model().objects.get(**res.data)
        self.assertTrue(user.check_password(self.payload['password']))
        self.assertNotIn('password', res.data)

    def test_user_exists(self):
        create_user(**self.payload)
        res = self.client.post(register_url, self.payload)
        self.assertEqual(res.status_code, status.HTTP_400_BAD_REQUEST)

    def test_password_too_short(self):
        payload = {'email': 'test@lokajbot.com', 'username': 'lalala', 'password': 'bee'}
        res = self.client.post(register_url, payload)
        self.assertEqual(res.status_code, status.HTTP_400_BAD_REQUEST)
        self.assertFalse(get_user_model().objects.filter(email=payload['email']).exists())

class UserTokenAuthTests(TestCase):

    def setUp(self):
        self.client = APIClient()

        self.create_payload = {
            'email': 'proztocki@lokajbot.com',
            'password': 'LewisHamilton44',
            'username': 'PRoztocki',
        }

        self.payload = {
            'password': 'LewisHamilton44',
            'username': 'PRoztocki',
        }

    def test_user_unauthorized_incorrect(self):
        res = self.client.post(token_url)
        self.assertEqual(res.status_code, status.HTTP_400_BAD_REQUEST)

    def test_create_token_for_user(self):
        create_user(**self.create_payload)
        res = self.client.post(token_url, self.payload)
        self.assertEqual(res.status_code, status.HTTP_200_OK)
        self.assertIn('token', res.data)

    def test_create_token_password_incorrect(self):
        create_user(**self.create_payload)
        wrong_payload = {'username': 'PRoztocki', 'password': 'MaxVerstappen33'}
        res = self.client.post(token_url, wrong_payload)
        self.assertEqual(res.status_code, status.HTTP_400_BAD_REQUEST)

    def test_create_token_no_user_incorrect(self):
        res = self.client.post(token_url, self.payload)
        self.assertEqual(res.status_code, status.HTTP_400_BAD_REQUEST)

    def test_create_token_missing_username_incorrect(self):
        wrong_payload = {'username': '', 'password': 'MaxVerstappen33'}
        res = self.client.post(token_url, wrong_payload)
        self.assertEqual(res.status_code, status.HTTP_400_BAD_REQUEST)

    def test_create_token_missing_password_incorrect(self):
        wrong_payload = {'username': 'PRoztocki', 'password': ''}
        res = self.client.post(token_url, wrong_payload)
        self.assertEqual(res.status_code, status.HTTP_400_BAD_REQUEST)
