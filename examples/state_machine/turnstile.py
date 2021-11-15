import os
from hypothesis import note, settings
from hypothesis.stateful import RuleBasedStateMachine, initialize, precondition, rule, invariant

def get_state_client():
    stream = os.popen('ANCHOR_WALLET=./keys/id.json node ./js_client/getState.js')
    output = stream.readlines()
    
    res = []
    for line in output:
        if line.strip() == "true":
            res.append(True)
        elif line.strip() == "false":
            res.append(False)
    
    # print("Locked: {}".format(res[0]))
    # print("Coin  : {}".format(res[1]))
    return (res[0], res[1])

def coin_client():
    stream = os.popen('ANCHOR_WALLET=./keys/id.json node ./js_client/coin.js')
    _ = stream.readlines()

def push_client():
    stream = os.popen('ANCHOR_WALLET=./keys/id.json node ./js_client/push.js')
    _ = stream.readlines()

def init_client():
    stream = os.popen('ANCHOR_WALLET=./keys/id.json node ./js_client/init.js')
    _ = stream.readlines()

class Turnstile(RuleBasedStateMachine):
    locked = True
    # lockedSUT = True
    # resSUT = False

    @rule()
    def coin(self):
        # inserting a coin is just calling coin
        coin_client()
        # self.lockedSUT = False
        
        # update
        self.locked = False
        
        # get current state
        (after, _) = get_state_client()
        # (after, _) = (self.lockedSUT, self.resSUT)
        
        # ensure that coin insert unlocks turnstile
        assert not(after)

    @precondition(lambda self: self.locked == True)
    @rule()
    def push_locked(self):
        # get before state
        (before, _) = get_state_client()
        # (before, _) = (self.lockedSUT, self.resSUT)

        # pushing is just calling push
        push_client()
        # self.lockedSUT = True
        # self.resSUT = False

        # get current state
        (after, res) = get_state_client()
        # (after, res) = (self.lockedSUT, self.resSUT)

        # update
        self.locked = True
        # ensure that pushing fails and turnstile is locked
        assert not(res) and after and before

    @precondition(lambda self: self.locked == False)
    @rule()
    def push_unlocked(self):
        # get before state
        (before, _) = get_state_client()
        # (before, _) = (self.lockedSUT, self.resSUT)

        # pushing is just calling push
        push_client()
        # self.lockedSUT = True
        # self.resSUT = True
        
        # get current state
        (after, res) = get_state_client()
        # (after, res) = (self.lockedSUT, self.resSUT)

        # update  
        self.locked = True
        # ensure that pushing fails and turnstile is locked
        assert res and after and not(before)


Turnstile.TestCase.settings = settings(max_examples=3, deadline=12000)

TurnstileTest = Turnstile.TestCase
