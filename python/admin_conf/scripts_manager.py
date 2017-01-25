import fnmatch
import os
import yaml


def replace_cores_in_scripts(core_path, old_core, new_core):
    matches = []
    for root, dirnames, filenames in os.walk(PATH):
        for filename in fnmatch.filter(filenames, '*.py'):
            script_path = os.path.join(root, filename)
            replace_core(script_path, old_core, new_core)

def replace_core(script_path, old_core, new_core):
    with open(script_path, 'r+') as script_file:
        print script_path
        script = script_file.read()
        script_file.seek(0)
        script_file.truncate() 
        script_file.write(script.replace('core%s' % old_core, 'core%s' % new_core))

def generate_yaml(conf_path):

    def save_script(script_path):
        #print script_path[-1].replace('.xml','')
        index = script_path[-1].find('.xml')
        return script_path[-1][:index]

    def save_folder(res, script_path):
        folder_name = script_path[0]
        if len(script_path[1:]) > 1:
            if folder_name not in res:
                res[folder_name] = {}
            save_folder(res[folder_name], script_path[1:])
        else:
            if folder_name not in res:
                res[folder_name] = []
            res[folder_name].append(save_script(script_path))

    with open(conf_path, 'r') as conf_file:
        res = {'NONE':[]}
        for line in conf_file.readlines():
            script_path = line.split('/')
            if len(script_path) > 1:
                save_folder(res, script_path)
            else:
                res['NONE'].append(save_script(script_path))

    with open('to_conf.yaml', 'w') as yaml_file:
        yaml.dump(res, yaml_file)


SCRIPTS_PATH = 'D:\\Documents\\Projects\\work\\kor_game_kingdom\\PythonScripts\\LogicServer\\runing\\service\\core29'
CONFIG_PATH = 'scripts_paths.txt'
#replace_cores_in_scripts(SCRIPTS_PATH, 24, 29)
generate_yaml(CONFIG_PATH)