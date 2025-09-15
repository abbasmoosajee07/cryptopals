
def build_cookie_object(info):
    cookie_dict = {}
    split_info = info.split("&")
    for entry in split_info:
        key, value = entry.split("=")
        cookie_dict[key] = value
    return cookie_dict

test_input = "foo=bar&baz=qux&zap=zazzle"
cookie = build_cookie_object(test_input)
print(cookie)

def profile_for(email, uid, role):
    email_dict = {
        "email": email,
        "uid": uid,
        "role": role,
    }
    return email_dict

def encode_profile(email_info, uid = 10, role = "user"):
    email_dict = profile_for(email=email_info, uid=uid, role=role)
    # email=foo@bar.com&uid=10&role=user
    print(email_dict)
    encoded_info = f"email={email_info}&uid={uid}&role={role}"
    return encoded_info

test_email = "foo@bar.com"
email_encoded = encode_profile(test_email)
print(email_encoded)