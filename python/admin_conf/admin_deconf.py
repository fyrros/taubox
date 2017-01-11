from lxml import etree as ET
from copy import copy

import yaml


class Script:

	pass


class Scripts(dict):

	REGULAR = {
		'auto': 'swag,combat,gamer,traders,planner,gift,cabmans,digest,corpse_manager,repair,resource_server'.split(','),
		'special': 'email,core_email,arena,arena_battle_ground,battle_ground'.split(',')
	}

	def add_script(self, script):
		if script.is_mob():
			self.__add_mob(script)
		else:
			self.__add_regular(script)

	def __add_mob(self, script):
		if script.mob_folder not in self['mobs']:
			self['mobs'][script.mob_folder] = []
		self.__add_script(self['mobs'][script.mob_folder], script)

	def __add_regular(self, script):
		for script_type, script_names in Scripts.REGULAR.items():
			if script.name in script_names:
				self.__add_script(self.regular[script_type], script)
			else:
				self.__add_script(self.regular['other'], script)

	def __add_script(self, target, script):
		target.append(script.res())



class Core:

	def __init__(self, group_xml):
		self.id = self.__get_core_id(group_xml)
		self.scripts = Scripts(auto=[], special=[], other=[], mobs={})
		self.__get_scripts(group_xml)

	def __get_core_id(self, group_xml):
		return int(group_xml.get('name').split('_')[0].replace('core',''))

	def __get_scripts(self, group_xml):
		for service_xml in group_xml:
			self.scripts.add_script(Script(service_xml))


if __name__ == '__main__':
	xml_conf = ET.parse('/home/fyrros/workspace/projects/kor_social_outer/data/srveye/templates.xml')
	yaml_res = '/home/fyrros/workspace/projects/ksibox/admin_conf/deconf.yaml'
	cores = [Core(group_xml) for group_xml in xml_conf.xpath('//services/group') if group_xml.get('name').split('_')[1] == '1']

	with open(yaml_res, 'w') as yaml_res_file:
		yaml.dump({core.id:core.scripts for core in cores}, yaml_res_file, default_flow_style=False)
