# -*- coding: utf8 -*-

import json
import redis

from sqlalchemy import create_engine
from sqlalchemy.engine.url import URL


class User:
    
    ATTRS = {'id', 'platform', 'gold', 'rubies', 'json'}

    def __init__(self, json_data: str):
        self.json = json_data.decode('utf-8')
        data = json.loads(self.json)
        data_user = data['user']

        self.id = data['login']
        self.platform = 0 if data['platform'] == 'vk' else 2
        #self.friends = data_user['friends']
        self.rubies = data_user['rubies']
        self.gold = data_user['gold']
        #self.tutorial_completed = data_user['tutorial_completed']
        #self.tutorial_checkpoint = data_user['tutorial_checkpoint']
        #self.workshop_level = data_user['workshop_level']
        #self.gold_mine_level = data_user['gold_mine_level']
        #self.rating = data_user['rating']
        #self.wins = data_user['battle_stats']['overall']['wins']
        #self.loses = data_user['battle_stats']['overall']['loses']
        #self.experience = data_user['experience']
        #self.rank = data_user['battle_stats']['tournament']['rank']

    @property
    def sql(self):
        return "({id}, {platform}, {rubies}, {gold}, '{json}')".format(**{a:getattr(self, a) for a in User.ATTRS})

class Users(list):
    
    def load_from_redis(self, redis_users: dict):
        for _, users in redis_users.items():
            for json_data in users:
                self.append(User(json_data))
        print('Users loaded from redis')

    @property    
    def sql(self) -> "SQL string":
        query = 'DELETE FROM saved_users;'
        query += 'INSERT INTO saved_users (user_id, platform, gold, rubies, json) VALUES %s'
        users_values = []
        for user in users:
            users_values.append(user.sql)
        return query % ','.join(users_values)


class RedisManager:
    
    def __init__(self):
        self._conns = {}
        for db in (0,2):
            self._conns[db] = redis.Redis(host='localhost', port=6379, db=db)
        print('Redis Manager initialized')

    def get_users(self):
        result = {}
        for db, conn in self._conns.items():
            result[db] = conn.mget(uid for uid in conn.keys() if uid.isdigit())
        return result


class PostgreSQLManager:
    
    def __init__(self):
        login = {
            'drivername': 'postgres',
            'host' : 'localhost',
            'port' : 5432,
            'username' : 'tm',
            'password' : 'tmpassword123',
            'database': 'tm'
        }
        self._engine = create_engine(URL(**login))
        print('DB Manager initialized')

    def update_users(self, sql):
        with self._engine.begin() as conn:
            conn.execute(sql)
        print('Data saved to DB')


if __name__ == '__main__':
    print('Starting')
    redis_manager = RedisManager()
    db_manager = PostgreSQLManager()
    users = Users()

    users.load_from_redis(redis_manager.get_users())
    db_manager.update_users(users.sql)
    print('Finished')