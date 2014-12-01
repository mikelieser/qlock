locale = 
  code: "formrausch"
  name: "formrausch"
  
  characters:  "ESRISTSFÜNF
                ZEHNZWANZIG
                DREIVIERTEL
                VøRFøRMNACH
                HALBRELFÜNF
                EINSAATZWEI
                DREIUSCVIER
                SECHSΣHACHT
                SIEBENZWøLF
                ZEHNEUNDUHR
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
    feierabend: [11, 25, 26, 29, 30, 61, 92, 93, 106, 107]
    formrausch: [37,38,39,40,49,60,71,72,73,84]
    happa: [27,28,29,38,39,40,49,50,51,60,61,62,71,72,73,82,83,84, 6, 105, 25,47,69, 86, 64, 42]
  
  setChars: (words, h, m, now) ->
    
    hh = parseInt(now.format("H"))

    # HAPPA
    if hh == 11 && m >= 15 && m <= 45 && m % 2 == 1
      $('.minute').addClass('on')
      qlock.enqueueCharacters words.happa
      $('#clock').data('title-time', 'FRÜHSTÜCKSZEIT ;)')
      return

    # FEIERABEND ;)
    if hh == 18
      $('.minute').addClass('on')
      $('#clock').data('title-time', 'FEIERABEND ;)')
      if m % 2 == 0
        qlock.enqueueCharacters words.intro
        qlock.enqueueCharacters words.feierabend
      else
        qlock.enqueueCharacters words.formrausch
      return

    # normal operating mode

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
      qlock.enqueueCharacters words["hours_#{qlock.helperNextHour(h)}"]
    else if m >= 30 && m < 35
      qlock.enqueueCharacters words.half
      qlock.enqueueCharacters words["hours_#{qlock.helperNextHour(h)}"]
    else if m >= 35 && m < 40
      qlock.enqueueCharacters words.minutes_5
      qlock.enqueueCharacters words.past
      qlock.enqueueCharacters words.half
      qlock.enqueueCharacters words["hours_#{qlock.helperNextHour(h)}"]
    else if m >= 40 && m < 45
      qlock.enqueueCharacters words.minutes_20
      qlock.enqueueCharacters words.before
      qlock.enqueueCharacters words["hours_#{qlock.helperNextHour(h)}"]
    else if m >= 45 && m < 50
      qlock.enqueueCharacters words.minutes_15
      qlock.enqueueCharacters words.before
      qlock.enqueueCharacters words["hours_#{qlock.helperNextHour(h)}"]
    else if m >= 50 && m < 55
      qlock.enqueueCharacters words.minutes_10
      qlock.enqueueCharacters words.before
      qlock.enqueueCharacters words["hours_#{qlock.helperNextHour(h)}"]
    else if m >= 55
      qlock.enqueueCharacters words.minutes_5
      qlock.enqueueCharacters words.before
      qlock.enqueueCharacters words["hours_#{qlock.helperNextHour(h)}"]

    return
    
qlock.addLocale locale