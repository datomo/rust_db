import mysql.connector
import time
import random

# database variables
user = 'root'
password = 'admin'
database = 'test'

def main():
    now = time.time()
    insert(10000000)
    #open_write()
    #insert_once(10000000, 1000000)
    new_now = time.time()
    print(new_now - now)

def open_write(): 
    
    file1 = open("../arcos_all_washpost.tsv", "r")

    out = open('out.txt', 'a+')

    counter = 0

    for _ in file1.readlines():
        counter += 1

    print("len is {}".format(counter))

    file1.close()
    file1 = open("../arcos_all_washpost.tsv", "r")

    i = 0

    while True:

        line = file1.readline()

        if not line:
            break
        
        out.write(",".join(line.strip().split("\t"))+ "]\n");

        if i%50000000 == 0:
            print("at line: {}".format(i))
        i += 1

    file1.close()

def insert(amount:int):
    print("hi")
    db = Database()

    db.query('''CREATE TABLE python_table (
                         customer_id int not null,
                         amount int not null,
                         account_name text
                     )''')

    db.commit()

    payments = []
    for i in range(amount):
        n = random.randint(1,20000)
        pay = Payment(i, n, None)
        payments.append( Payment.to_list(pay))

    db.querymany("INSERT INTO python_table VALUES(%s,%s,%s)", payments)

    db.commit()
    db.close_connection()


def insert_once(amount:int, chunk:int):

    db = Database()

    db.query('''CREATE TABLE python_table (
                         customer_id int not null,
                         amount int not null,
                         account_name text
                     )''')

    db.commit()

    payments = "INSERT INTO python_table VALUES"

    executed = False

    for i in range(amount):
        n = random.randint(1,20000)
        pay = Payment(i, n, None)
        #payments.append( Payment.to_list(pay))
        if i != 0 and not executed:
            payments += ",({}, {}, {})".format(i, n, 0)
        else:
            payments += "({}, {}, {})".format(i, n, 0)

        executed = False

        if i%1000000 == 0:
            print("at line: {}".format(i))
            db.query(payments)
            db.commit()
            payments = "INSERT INTO python_table VALUES"

            executed = True

    if not executed:
        db.query(payments)
        db.commit()
    

    

    

class Payment: 
    def __init__(self, customer_id, amount, account_name):
        self.customer_id = customer_id
        self.amount = amount
        self.account_name = account_name

    @staticmethod
    def to_list(payement):
        return [payement.customer_id, payement.amount, payement.account_name]

class Database:
    def __init__(self):
        self.open_connection()
        self.cursor = self.db.cursor()
        # self.query("SET FOREIGN_KEY_CHECKS=0;")
        self.buffered_cursor = self.db.cursor(buffered=True)

    def open_connection(self):
        self.db = mysql.connector.connect(
            host="localhost",
            port="3306",
            user=user,
            passwd=password,
            database=database,
            connect_timeout=28800,
            allow_local_infile=True
        )

    def commit(self):
        self.db.commit()

    def query(self, query):
        try:
            self.cursor.execute(query)
            self.db.commit()
        except mysql.connector.Error as e:
            print("Something went wrong: {}".format(e))
            #print(query)

    def querymany(self, query, data_list) -> str:
        try:
            self.cursor.executemany(query, data_list)
            self.db.commit()
        except mysql.connector.Error as e:
            print("Something went wrong: {}".format(e))
            print(query)
            return "error"

    
    def close_connection(self):
        self.cursor.close()
        self.buffered_cursor.close()
        self.db.close()
        # self.tunnel.stop()


if __name__ == "__main__":
    main()