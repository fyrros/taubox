from lxml import etree as ET


class Floor:

    def __init__(self, num):
        self.num = num

    def set_points(self, points):
        self.points = ','.join(points.split())

    def __repr__(self):
        return f'{self.num}::{self.points}\n'

    def __str__(self):
        return self.__repr__()


if __name__ == '__main__':
    floors = []

    with open('target.xml') as target_file:
        target_xml = ET.fromstring(target_file.read())

    for floor_el in target_xml:
        num = floor_el.get('data-floor')
        floors.append(Floor(num))
        for polygon in floor_el:
            floors[-1].set_points(polygon.get('points'))

    with open('result.txt', 'w') as res:
        res.write(''.join(str(f) for f in floors))
