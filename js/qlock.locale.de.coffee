locale = 
  code: "de"
  name: "german"
  
  characters:  "
                ESKISTAFÜNF
                ZEHNZWANZIG
                DREIVIERTEL
                VORFUNKNACH
                HALBAELFÜNF
                EINSXÄMZWEI
                DREIAUJVIER
                SECHSNLACHT
                SIEBENZWÖLF
                ZEHNEUNKUHR
               "
               
  words:
    intro: [1,2,4,5,6]
    before: [34..36]
    past: [41..44]
    half: [45..48]
    hour: [108..110]
    minutes_5: [8..11]
    minutes_10: [12..15]
    minutes_15: [27..33]
    minutes_20: [16..22]
    minutes_45: [23..33]
    hours_1: [56..59]
    hours_1s: [56..58]
    hours_2: [63..66]
    hours_3: [67..70]
    hours_4: [74..77]
    hours_5: [52..55]
    hours_6: [78..82]
    hours_7: [89..94]
    hours_8: [85..88]
    hours_9: [103..106]
    hours_10: [100..103]
    hours_11: [50..52]
    hours_12: [95..99]
    
  setChars: (words, h, m, now) ->
    
    # 14.00 - 14.04 => es ist drei uhr
    # 14.05 - 14.09 => es ist fünf nach drei
    # 14.10 - 14.14 => es ist zehn nach drei
    # 14.15 - 14.19 => es ist viertel nach drei
    # 14.20 - 14.24 => es ist zwanzig nach drei
    # 14.25 - 14.29 => es ist fünf vor halb drei
    # 14.30 - 14.34 => es ist halb drei
    # 14:35 - 14.39 => es ist fünf nach halb drei
    # 14.40 - 14.44 => es ist zwanzig vor drei
    # 14.45 - 14.49 => es ist viertel vor drei
    # 14.50 - 15.54 => es ist zehn vor drei
    # 14.55 - 14.59 => es ist fünf vor drei

    # es ist ein uhr / es ist viertel nach eins (einzige ausnahme?)

    # intro
    qlock.enqueueCharacters words.intro if qlock.config.clock_intro

    # time
    if m >= 0 && m < 5
      chars_hours = words["hours_#{h}"]
      chars_hours = words["hours_1s"] if h == 1
      qlock.enqueueCharacters chars_hours
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
      qlock.enqueueCharacters words.minutes_5
      qlock.enqueueCharacters words.before
      qlock.enqueueCharacters words.half
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]
    else if m >= 30 && m < 35
      qlock.enqueueCharacters words.half
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]
    else if m >= 35 && m < 40
      qlock.enqueueCharacters words.minutes_5
      qlock.enqueueCharacters words.past
      qlock.enqueueCharacters words.half
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]
    else if m >= 40 && m < 45
      qlock.enqueueCharacters words.minutes_20
      qlock.enqueueCharacters words.before
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]
    else if m >= 45 && m < 50
      qlock.enqueueCharacters words.minutes_15
      qlock.enqueueCharacters words.before
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]
    else if m >= 50 && m < 55
      qlock.enqueueCharacters words.minutes_10
      qlock.enqueueCharacters words.before
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]
    else if m >= 55
      qlock.enqueueCharacters words.minutes_5
      qlock.enqueueCharacters words.before
      qlock.enqueueCharacters words["hours_#{qlock.helper.nextHour(h)}"]

    return
    

qlock.addLocale locale