transition = 
  name: "explode"
  
  method: (all_old_chars, all_new_chars, old_chars, new_chars) ->
    
    qlock.log(old_chars)
    qlock.log(new_chars)  
    
    jquery_all_old_chars = qlock.helper.transition.buildElements(all_old_chars)
    jquery_all_new_chars = qlock.helper.transition.buildElements(all_new_chars)
    jquery_old_chars = qlock.helper.transition.buildElements(old_chars)
    jquery_new_chars = qlock.helper.transition.buildElements(new_chars)
    
    
    #jquery_all_old_chars.removeClass('on')
    #jquery_all_new_chars.addClass('on')
    
    jquery_all_old_chars.velocity
          colorAlpha: 0.14
          complete: (elements) ->
              return
              
    # jquery_all_old_chars.velocity
    #       translateX: "200px"
    #       rotateZ: "45deg"
    #       translateZ: "323px"
    #       complete: (elements) ->
    #           return
    #
              
    
    jquery_all_new_chars.velocity
          colorAlpha: 1
          complete: (elements) ->
              return
                  
    return
    

qlock.addTransition transition