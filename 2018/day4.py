import re
from datetime import date, timedelta


class Calender:
    def __init__(self):
        self.calender = {}

    def add_day(self, s_date):
        ''' adds the date if it doesnt already exist, otherwise adds whatever action to that date '''
        pattern = re.compile(r'\[1518-(\d+)-(\d+) (\d\d):(\d+)\] (Guard|falls|wakes) (#\d*)?')
        m, d, hours, mins, action, num = pattern.search(s_date).groups()
        day = date(1518, int(m), int(d))

        # inc the date if the shift started just before midnight
        if hours != '00':
            day = day + timedelta(days=1)
            
        if day in self.calender.keys():
            if action == 'Guard':
                self.calender[day][1] = num
            else:
                self.calender[day][0][int(mins)] = action[0]
        else:
            self.calender[day] = [['w'] + ['.' for __ in range(59)], num]

    def render(self):
        print('Date       ID   Minute')
        print(' '*15 + '000000000011111111112222222222333333333344444444445555555555')
        print(' '*15 + '012345678901234567890123456789012345678901234567890123456789')
        for d in self.calender.keys():
            _id = self.calender[d][1]
            print(f'{d} {_id} {"".join(self.calender[d][0])}')


class Guard:
    guards = []
    
    def __init__(self, _id):
        self._id = _id
        self.total_sleep = 0
        self.slept_min = []
        self.shifts = []
        Guard.guards.append(self)

    @classmethod
    def add_record(cls, _id, record):
        # find guard obj or create a new one
        for g in cls.guards:
             if g._id == _id:
                 guard = g
                 break
        else:
            guard = cls(_id)

        guard.shifts.append(record)
        
        # increase guards mins slept
        cur = ''
        for _min, i in enumerate(record):
            if i == 'w' or i == 'f':
                cur = i
            if cur == 'f':
                guard.total_sleep += 1
                guard.slept_min.append(_min)

    @classmethod
    def sleepiest_guard(cls):
        ''' returns the guard that slept the most '''
        guard = None
        most_sleep = 0
        for g in cls.guards:
            if g.total_sleep > most_sleep:
                most_sleep = g.total_sleep
                guard = g
        guard.render()
        return guard

    @classmethod
    def sleepiest_minute(cls):
        ''' returns the guard which is most frequently asleep on the same minute '''
        guard = None
        min_count = 0
        _min = 0
        
        for g in cls.guards:
            m = g.most_slept_min()
            c = g.min_sleep_count(m)
            if c > min_count:
                guard = g
                _min = m
                min_count = c

        guard.render()
        return guard

    def min_sleep_count(self, _min):
        ''' returns the count of the times the guard was asleep during that min '''
        return self.slept_min.count(_min)

    def most_slept_min(self):
        ''' returns the min value that the guard was most often asleep '''
        try:
            return max(set(self.slept_min),
                       key=self.slept_min.count)
        except:
            return None

    def render(self):
        print('='*50)
        print(f'guard id: {self._id}')
        print(f'total sleep: {self.total_sleep}')
        print(f'most common min: {self.most_slept_min()}')
        print(f'most common min count: {self.min_sleep_count(self.most_slept_min())}')
        print(f'answer: { int(self._id[1:]) * self.most_slept_min()}')

    def render_shifts(self):
        print('000000000011111111112222222222333333333344444444445555555555')
        print('012345678901234567890123456789012345678901234567890123456789')
        asleep = False
        for shift in self.shifts:
            for i in shift:
                if i == 'w':
                    asleep = False
                elif i == 'f':
                    asleep = True
                if asleep:
                    print('#', end='')
                else:
                    print('.', end='')
            print('')
            asleep = False


def main():
    with open('day4.txt', 'r') as data:
        c = Calender()
        
        for d in data.readlines():
            d = d.rstrip()
            c.add_day(d)

        for record, _id in c.calender.values():
            Guard.add_record(_id, record)

        sleepy = Guard.sleepiest_minute()


main()
