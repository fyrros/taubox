import os
import yaml

class Core:
    
    def __init__(self, core_num, core):
        self.num = core_num
        self.port = core['port']
        self.mirrors = core['mirrors']
        self.scripts = {}
        
    def add_script(self, script):
        self.scripts[script.name] = script

class Script:
    
    def __init__(self, script_name, folders = None):
        self.name = script_name
        self.folders = folders or []
        self.cores = {}
        
        self.logic_name = self.name
        self.fullpath = ''
        self.descr = ''
        
        self.__configure()
        
    def __configure(self):
        fullpath = ''
        for f in self.folders:
            fullpath += '%s/' % f
        self.fullpath = fullpath + self.name
        descr = '_'.join(self.folders + [self.name])
        words = []
        last_word = ''
        for word in descr.split('_'):
            if last_word.lower() == word or word == 'mob':
                continue
            last_word = word[0].upper() + word[1:]
            words.append(word[0].upper() + word[1:])
        self.descr = ' '.join(words)
        
        if 'mobs' in self.folders or 'quests' in self.folders:
            self.logic_name = '%s_%s' % (self.folders[0][:-1], self.name)
            if self.logic_name.count('mob') > 1:
                self.logic_name = self.logic_name.replace('_mob','',1)
                
        
    def __str__(self):
        print '='*16
        print self.cores
        print self.name
        print self.folders
        return ''

    def __len__(self):
        return len(self.folders) + len(''.join(self.folders))
        

class AdminConf:
    
    def __init__(self, conf_path):
        self.conf_path = conf_path
        self.cores = {}
        self.scripts = {}
        self.__configure()
    
    def __find_script(self, dir_name, dir, core_num, folders = None):
        if dir_name != 'NONE':
            if folders is None:
                folders = []
            folders = folders[:]
            folders.append(dir_name)
        if type(dir) == list:
            [self.__add_script(core_num, script_name, folders) for script_name in dir]
        elif type(dir) == dict:
            [self.__find_script(obj_name, obj, core_num, folders) for obj_name, obj in dir.items()]
    
    def __add_script(self, core_num, script_name, folders = None):
        if script_name not in self.scripts:
            script = self.scripts[script_name] = Script(script_name, folders)
        else:
            script = self.scripts[script_name]
        self.cores[core_num].add_script(script)
    
    def __configure(self):
        with open(self.conf_path + 'kor_conf.yaml', 'r') as main_conf:
            conf = yaml.load(main_conf)
        with open(self.conf_path + 'template.xml', 'r') as template:
            self.template = template.read()
        
        for core_num, core in conf['cores'].items():
            self.cores[core_num] = Core(core_num, core)
        
        for core_num, dirs in conf['logic'].items():
            [self.__find_script(dir_name, dir, core_num) for dir_name, dir in dirs.items()]

    def generate_config(self):
        with open(self.conf_path + 'result.xml', 'w') as result:
            for core in self.cores.values():
                for mirror_num, ip in core.mirrors.items():
                    data = {'ip': ip,
                            'num': mirror_num,
                            'core': core.num,
                            'port': core.port}
                    scripts = core.scripts.values()
                    scripts.sort(key = lambda s: len(s))
                    for script in scripts:
                        data['logic'] = script.logic_name
                        data['fullpath'] = script.fullpath
                        data['descr'] = script.descr
                        result.write(self.template % data)

# '/home/fyrros/workspace/projects/ksibox/admin_conf/'
    
if __name__ == '__main__':
    conf_path = ''
    app = AdminConf(conf_path)
    app.generate_config()
            
