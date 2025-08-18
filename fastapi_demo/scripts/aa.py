from apps.users.auth import get_password_hash

hasded_password = get_password_hash("aa246369")

print(hasded_password)