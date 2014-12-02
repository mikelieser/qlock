locale = 
  code: "ticktack"
  name: "ticktack"
  
  characters:  "
                TICK
                TACK
               "
  
  words:
    tick: [1..4]
    tack: [5..8]
  
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