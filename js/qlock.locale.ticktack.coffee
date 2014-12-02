locale = 
  code: "ticktack"
  name: "ticktack"
  
  characters:  "
                ES
                IST
                TICK
                TACK
                UHRZEIT
               "
  
  words:
    tick: [6..9]
    tack: [10..13]
  
  setChars: (words, h, m, now) ->
    
    s = parseInt(now.format("s"))
    
    if s % 2 == 0
      $('.minute').addClass('on')
      qlock.enqueueCharacters words.tick
      qlock.setTitle "TICK"
    else
      $('.minute').removeClass('on')
      qlock.enqueueCharacters words.tack
      qlock.setTitle "TACK"
    
    return
    
qlock.addLocale locale