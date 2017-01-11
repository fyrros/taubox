import redis
import progressbar

class FindGoodUsers:

	def __init__(self):
		r = redis.StrictRedis(host='localhost', port=6379, db=0)

		self.users = [k for k in r.keys() if k.isdigit()]
		print 'users loaded'
		self.result = []

		def filter_arenas(a):
			if ('find_bot' in a) or ('find_opponent' in a):
				return True
			return False

		self.arenas = [a for a in r.lrange('arenas_log', 0, -1) if filter_arenas(a)]
		print 'arenas loaded'
		self.battles = r.hgetall('battles').keys()
		print 'battles loaded'

		self.bar = progressbar.ProgressBar(maxval=len(self.users), widgets=[progressbar.Bar('=', '[', ']'), ' ', progressbar.Percentage()])

	def good_user(self, u):
		for a in self.arenas:
			if u in a:
				return True
		for b in self.battles:
			if u in b:
				return True
		return False

	def start(self):
		self.bar.start()
		i = 0

		for user in self.users:
			i += 1
			if self.good_user(user):
				self.result.append(user)
			self.bar.update(i)

		with open('users_stat.dat', 'w') as user_file:
			user_file.write(','.join(self.result))

		self.bar.finish()

		len_u = len(self.users)
		len_r = len(self.result)
		percent = float(len_r)/float(len_u)*100.0
		print len_r
		print len_u
		print percent
		res = 'Result : %s out of %s (%s)' % (str(len_r), str(len_u), str(percent))
		print res

fgu = FindGoodUsers()
fgu.start()