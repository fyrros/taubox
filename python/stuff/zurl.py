import requests
import argparse


URL = 'http://109.234.154.49:1620/command'


class Tester:

    METHODS = {
        0 : 'get_userinfo',
        1 : 'get_battle_state2',
        2 : 'play_cards2',
        3 : 'get_stroke_result2'
    }

    PARAMS = {
        'play_cards2': {
            "owner":{"hand":{},"ability":{}},
            "turn":6,
            "enemy":{"hand":{},"ability":{}},
            "discard":[]
        } 
    }


    def __init__(self):
        self.data = {
            'header': {
                'method': 'get_userinfo',
                'auth': '6c1697e1bab3be0ee163dc1459ba5608',
                'login': 1223148
            }
        }

    def _get_data(self, case):
        method = Tester.METHODS[case]
        self.data['header']['method'] = method
        if method in Tester.PARAMS:
            self.data.update(Tester.PARAMS[method])
            if method == 'play_cards2':
                with open('turn.dat', 'r+') as turn_file:
                    self.turn = int(turn_file.read())
                    turn_file.seek(0)
                    turn_file.write(str(self.turn+1))
                self.data['turn'] = self.turn

        return self.data

    def run(self, case):
        data = self._get_data(case)
        print data
        response = requests.post(URL, json = data)
        print response.text
        

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('-c', "--case", help="choose test case")
    args = parser.parse_args()

    case = int(args.case) or 0

    r = Tester()
    r.run(case)
