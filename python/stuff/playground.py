from enum import Enum

class NodeTypes(Enum):

    LIST = list
    DICT = dict
    TEXT = str

x = dict(spam = 1, ham = 2)
y = [0,2,3]

node_types = {
    NodeTypes.LIST: 
}