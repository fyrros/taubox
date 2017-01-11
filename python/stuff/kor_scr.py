import os
import yaml

root = '/home/fyrros/workspace/projects/kor_game2/PythonScripts/LogicServer/runing/service/'
res = {'27': {'NONE': []}, '28': {'NONE':[]}}

for core in res:
	path = '%score%s' % (root, core)
	for ff in os.listdir(path):
		if '.py' in ff:
			res[core]['NONE'].append(ff.split('.py')[0])
		else:
			path2 = '%s/%s' % (path, ff)
			if ff == 'mobs':
				res[core][ff] = {}
				for fff in os.listdir(path2):
					path3 = '%s/%s' % (path2, fff)
					res[core][ff][fff] = []
					for ffff in os.listdir(path3):
						res[core][ff][fff].append(ffff.split('.py')[0])
			else:
				res[core][ff] = []
				for fff in os.listdir(path2):
					res[core][ff].append(fff.split('.py')[0])

result = yaml.dump(res)
with open('konf.yaml', 'w') as y:
	y.write(result)