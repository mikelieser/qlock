locale = 
  code: "en"
  name: "english"
  
  characters:  "
                ITLISASTIME
                ACQUARTEROC 
                TWENTYFIVEX
                HALFBTENFTO
                PASTERUNINE
                ONESIXTHREE
                FOURFIVETWO
                EIGHTELEVEN
                SEVENTWELVE
                TENSEOCLOCK
               "
               
  words:
    intro: [1,2,4,5]
    half: [34..37]
    past: [45..48]
    hour: [105..110]
    to: [43..44]
    minutes_5: [29..32]
    minutes_10: [39..41]
    minutes_15: [14..20]
    minutes_20: [23..28]
    minutes_25: [23..32]
    hours_1: [56..58]
    hours_2: [75..77]
    hours_3: [62..66]
    hours_4: [67..70]
    hours_5: [71..74]
    hours_6: [59..61]
    hours_7: [89..93]
    hours_8: [78..82]
    hours_9: [52..55]
    hours_10: [100..102]
    hours_11: [83..88]
    hours_12: [94..99]
    
  setChars: (words, h, m, now) ->
    
    # intro
    qlock.enqueueCharacters words.intro if qlock.config.clock_intro

    # time
    if m >= 0 && m < 5
      qlock.enqueueCharacters words["hours_#{h}"]
      qlock.enqueueCharacters words.hour
    else if m >= 5 && m < 10
      qlock.enqueueCharacters words.minutes_5
      qlock.enqueueCharacters words.past
      qlock.enqueueCharacters words["hours_#{h}"]
    else if m >= 10 && m < 15
      qlock.enqueueCharacters words.minutes_10
      qlock.enqueueCharacters words.past
      qlock.enqueueCharacters words["hours_#{h}"]
    else if m >= 15 && m < 20
      qlock.enqueueCharacters words.minutes_15
      qlock.enqueueCharacters words.past
      qlock.enqueueCharacters words["hours_#{h}"]
    else if m >= 20 && m < 25
      qlock.enqueueCharacters words.minutes_20
      qlock.enqueueCharacters words.past
      qlock.enqueueCharacters words["hours_#{h}"]
    else if m >= 25 && m < 30
      qlock.enqueueCharacters words.minutes_25
      qlock.enqueueCharacters words.past
      qlock.enqueueCharacters words["hours_#{h}"]
    else if m >= 30 && m < 35
      qlock.enqueueCharacters words.half
      qlock.enqueueCharacters words.past
      qlock.enqueueCharacters words["hours_#{h}"]
    else if m >= 35 && m < 40
      qlock.enqueueCharacters words.minutes_25
      qlock.enqueueCharacters words.to
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]
    else if m >= 40 && m < 45
      qlock.enqueueCharacters words.minutes_20
      qlock.enqueueCharacters words.to
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]
    else if m >= 45 && m < 50
      qlock.enqueueCharacters words.minutes_15
      qlock.enqueueCharacters words.to
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]
    else if m >= 50 && m < 55
      qlock.enqueueCharacters words.minutes_10
      qlock.enqueueCharacters words.to
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]
    else if m >= 55
      qlock.enqueueCharacters words.minutes_5
      qlock.enqueueCharacters words.to
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]

    return
    
qlock.addLocale locale