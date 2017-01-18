from lxml import etree as ET
from copy import copy

import yaml
import argparse


class Script:

    def __init__(self, service_xml):
        self.__xml = service_xml
        self._name = ''
        self._descr = ''
        self._path = ''
        self._mob_folder = ''

        self.__set_name()
        self.__set_descr()
        self.__set_path()

    def __set_name(self):
        self._name = '_'.join(self.__xml.get('name').split('_')[2:])

    def __set_descr(self):
        self._descr = self.__xml.get('descr')

    def __set_path(self):
        path_param = self.__find_path_param()
        self._path = self.__extract_path_and_mob_folder(path_param)

    def __find_path_param(self):
        for param in self.__xml:
            param_value = param.get('value')
            if param_value and param_value.endswith('.py'):
                return param_value

    def __extract_path_and_mob_folder(self, path_param):
        path = path_param.split('/')
        for i, path_part in enumerate(path):
            if path_part.startswith('service'):
                path = '/'.join(path[i+2:])
                if path.startswith('mobs'):
                    self.__set_mob_folder(path)
                break
        return path

    def __set_mob_folder(self, path):
        mob_folder = path.split('/')[1]
        mob_folder = mob_folder if not mob_folder.endswith('.py') else 'ROOT'
        self._mob_folder = mob_folder

    def is_mob(self):
        return bool(self._mob_folder)

    def res(self):
        return '%s (%s) @ %s' % (self._name, self._descr, self._path)

    @property
    def name(self):
        return self._name

    @property
    def mob_folder(self):
        return self._mob_folder


class Scripts(dict):

    REGULAR = {
        'auto': 'swag,combat,gamer,traders,planner,gift,cabmans,digest,corpse_manager,repair,resource_server'.split(','),
        'special': 'email,core_email,arena,arena_battle_ground,battle_ground'.split(',')
    }

    def __init__(self):
        super(self.__class__, self).__init__(auto=[], special=[], other=[], mobs={})

    def add_script(self, service_xml):
        if self.__valid_xml(service_xml):
            script = Script(service_xml)
            if script.is_mob():
                self.__add_mob(script)
            else:
                self.__add_regular(script)

    def __valid_xml(self, service_xml):
        return bool(service_xml.get('name'))

    def __add_mob(self, script):
        self.__prepare_mob_folder(script.mob_folder)
        self.__add_script(self['mobs'][script.mob_folder], script)

    def __prepare_mob_folder(self, mob_folder):
        if mob_folder not in self['mobs']:
            self['mobs'][mob_folder] = []

    def __add_regular(self, script):
        for script_type, script_names in Scripts.REGULAR.items():
            if script.name in script_names:
                self.__add_script(self[script_type], script)
                break
        else:
            self.__add_script(self['other'], script)

    def __add_script(self, target, script):
        target.append(script.res())

    @property
    def mobs(self):
        return self['mobs']


class Core:

    def __init__(self, group_xml):
        self.id = self.__get_core_id(group_xml)
        self.scripts = Scripts()
        self.__get_scripts(group_xml)

    def __get_core_id(self, group_xml):
        return int(group_xml.get('name').split('_')[0].replace('core',''))

    def __get_scripts(self, group_xml):
        for service_xml in group_xml:
            self.scripts.add_script(service_xml)

    def get_mobs(self):
        return self.scripts.mobs

    def dump(self, types):
        return {k:v for k,v in self.scripts.items() if (k in types) or not types}


class Cores(list):

    def __init__(self, xml_conf):
        super(self.__class__, self).__init__()
        for group_xml in self.__groups(xml_conf):
            if self.__correct_group_name(group_xml):
                self.append(Core(group_xml))

    def __correct_group_name(self, group_xml):
        return group_xml.get('name').split('_')[1] == '1'

    def __groups(self, xml_conf):
        return xml_conf.xpath('//services/group')

    def dump(self, types):
        types = types and types.split(',') or []
        return {core.id:core.dump(types) for core in self}


class Manager:

    def __init__(self, location):
        self.error = False
        path_config = self.__get_path_config(location)
        if not self.error:
            self.xml_conf = ET.parse(path_config)
            self.yaml_res_cores_path = 'result/cores.yaml'
            self.yaml_res_patterns_path = 'result/patterns.yaml'
            self.cores = Cores(self.xml_conf)

    def __get_path_config(self, location):
        with open('deconf_path.yaml', 'r') as deconf_yaml_file:
            config = yaml.load(deconf_yaml_file)
            if location in config:
                return config[location]
            else:
                self.error = True

    def save_cores(self, types):
        print 'saving cores ...'
        with open(self.yaml_res_cores_path, 'w') as yaml_res_cores_file:
            yaml.dump(self.cores.dump(types), yaml_res_cores_file, default_flow_style=False)

            #yaml.dump({core.id:(core.scripts if not special else core.scripts['special']) for core in self.cores},
            #    yaml_res_cores_file, default_flow_style=False)

    def save_patterns(self):
        print 'saving patterns ...'
        with open(self.yaml_res_patterns_path, 'w') as yaml_res_patterns_file:
            patterns = self.__get_patterns()
            yaml.dump(patterns, yaml_res_patterns_file, default_flow_style=False)

    def __get_patterns(self):
        res = {}
        for core in self.cores:
            mobs = core.get_mobs()
            for mob_folder in mobs:
                if mob_folder not in res:
                    res[mob_folder] = {'count':0, 'mobs':set(mobs[mob_folder])}
                res[mob_folder]['count'] += 1
                matched_mobs = res[mob_folder]['mobs'] & set(mobs[mob_folder])
                if res[mob_folder]['count'] > 1 and not matched_mobs:
                    matched_mobs = res[mob_folder]['mobs']
                res[mob_folder]['mobs'] = matched_mobs
        self.__convert_sets_to_lists(res)
        return res

    def __convert_sets_to_lists(self, res):
        for mob_folder_name, mob_folder in res.items():
            res[mob_folder_name]['mobs'] = list(mob_folder['mobs'])


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('-L', "--location", help="path config location")
    parser.add_argument('-T', "--types", help="save only target types")
    args = parser.parse_args()
    location = args.location or 'default'
    types = args.types or ''

    manager = Manager(location)
    if manager.error:
        print 'WRONG_LOCATION_ERROR'
    else:
        manager.save_cores(types)
        manager.save_patterns()