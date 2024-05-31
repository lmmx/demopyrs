import demopyrs


def test_version():
    print(demopyrs.__version__)


def test_client():
    client = demopyrs.create_api_client()
