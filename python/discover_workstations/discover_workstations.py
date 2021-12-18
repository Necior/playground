from ipaddress import ip_address
from pprint import pprint
from time import sleep

import zeroconf


def format_addresses(info):
    return "    ".join(str(ip_address(a)) for a in info.addresses)


class Listener:
    def __init__(self):
        self.devices = set()

    def add_service(self, zc, type_, name):
        info = zc.get_service_info(type_, name)
        self.devices.add((name, format_addresses(info)))

    def update_service(self, *args, **kwargs):
        pass


def discover(timeout):
    listener = Listener()
    browser = zeroconf.ServiceBrowser(
        zeroconf.Zeroconf(), "_workstation._tcp.local.", listener
    )
    sleep(timeout)
    browser.cancel()
    return listener.devices


def main():
    pprint(discover(3))


if __name__ == "__main__":
    main()
