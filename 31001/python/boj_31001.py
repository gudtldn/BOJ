# https://www.acmicpc.net/problem/31001

import sys
from collections import defaultdict
from dataclasses import dataclass
from decimal import Decimal, ROUND_DOWN

class StockMarket:
    def __init__(self):
        self.companies: dict[str, Company] = {}
        self.groups: dict[int, list[str]] = defaultdict(list)

    def add_company(self, group: int, name: str, price: int):
        self.companies[name] = Company(group, name, price)
        self.groups[group].append(name)

    # change_price
    def _3(self, stock_name: str, diff: int):
        self.companies[stock_name].change_price(diff)

    # change_group_price
    def _4(self, group: int, diff: int):
        for stock_name in self.groups[group]:
            self.companies[stock_name].change_price(diff)

    # change_group_price_percent
    def _5(self, group: int, percent: int):
        for stock_name in self.groups[group]:
            self.companies[stock_name].change_price_percent(percent)

@dataclass
class Company:
    group: int
    name: str
    price: int

    def change_price(self, diff: int):
        self.price += diff

    def change_price_percent(self, percent: int):
        self.price = ((self.price * (100 + percent)) // 100) // 10 * 10

class Investor:
    def __init__(self, money: int):
        self.money: int = money
        self.stock_info: dict[str, int] = defaultdict(int)

    # buy
    def _1(self, company: Company, amount: int):
        if self.money < company.price * amount:
            return

        self.money -= company.price * amount
        self.stock_info[company.name] += amount

    # sell
    def _2(self, company: Company, amount: int):
        fixed_amount = min(amount, self.stock_info[company.name])
        self.money += company.price * fixed_amount
        self.stock_info[company.name] -= fixed_amount

    # print_money
    def _6(self, *args):
        print(self.money)

    # print_asset
    def _7(self, companies: dict[str, Company]):
        asset = self.money + \
            sum([amount * companies[stock_name].price
                    for stock_name, amount in self.stock_info.items()])
        print(asset)


input = sys.stdin.readline
n, money, q = map(int, input().split())

stock_market = StockMarket()
investor = Investor(money)
for _ in range(n):
    group, stock_name, price = input().split()
    stock_market.add_company(int(group), stock_name, int(price))

for _ in range(q):
    query = list(map(lambda x: int(x) if x.isnumeric() or x[0] == "-" else x, input().split()))
    match query:
        case [(1 | 2) as menu, stock_name, amount]:
            getattr(investor, f"_{menu}")(stock_market.companies[stock_name], amount)
        case [(3 | 4 | 5) as menu, *args]:
            getattr(stock_market, f"_{menu}")(*args)
        case [(6 | 7) as menu]:
            getattr(investor, f"_{menu}")(stock_market.companies)
