qlock = 

  new_chars: [] # queue for new active chars
  old_chars: [] # queue for old active chars
  last_m: -1 # marker last minute
  last_h: -1 # marker last hour
  last_moment: null # last moment (demo mode)
  config: null # merged user params with defaults
  locales: [] # array of locales
  locale: null # active locale
  
  defaults:
    mode: "standard" # standard, all_words, demo
    clock_interval: 1000 # ms, 1000 is fine, increase for better performance and less efficiency
    demo_interval: 4000 # ms, if mode is "demo"
    demo_step: "random" # in minutes (1, 5, 60 or "random"), if mode is "demo"
    debug: false
    locale: "en" # en, de, formrausch
    theme: ["standard"] # standard, blue, red, gradient, timeonly, formrausch (or array for random theme)
  
    clock_intro: true # true, false (it is, es ist)
    clock_size: 0.8 # 0.5 - 0.9 percentage of window
    clock_padding: 0.14 # 0.0 - 0.3 (percentage padding to margins in clock)
    clock_char_size: 0.65 # 0.3 - 1 (percentage char size in clock)
    clock_minutes: true # true, false (show or hide minute dots)
    clock_minute_size: 0.4 # 0.2 - 0.6 (percentage minute dot size)
    clock_titletime: true # show time in doc/tab title (true or false)
    clock_start_animation: "zoomIn" # none, fadeInDown, bounceInDown, rotateIn, zoomIn, zoomInUp
  
  log: (msg) ->
    console.log(msg) if this.config.debug
    return
  
  init: (params) ->
    
    # merge defaults with user params
    this.config = $.extend this.defaults, params
    this.log this.config
    
    # set locale
    for l in this.locales
      if l.code == this.config.locale
        this.locale = l
        break
        
    if !this.locale
      this.log "locale #{this.config.locale} not found"
      return
      
    this.log "locale: #{this.locale.code} (#{this.locale.name})"
    
    # disable css transitions on page load
    $('body').addClass('no-transitions')

    this.fillClockCharacters()
    this.setClockSize()
    theme = this.config.theme # one theme
    theme = this.config.theme[this.helperRandomInt(0, theme.length-1)] if typeof this.config.theme is 'object' # array of themes, get one by random
    $('body').addClass("theme-#{theme}")
    $('#clock_wrapper').addClass("animated #{this.config.clock_start_animation}") if this.config.clock_start_animation != "none"
    
    if this.config.mode == "all_words"
      this.activateAllWords() # show all used chars
    else
      this.startTimer()

    # enable css transitions after on page load
    $(window).load ->
      $('body').removeClass('no-transitions')
      return

    # handle clock on window resize
    $(window).resize ->
      qlock.setClockSize()
      return
    
    return
    
    

  fillClockCharacters: ->
    $('#chars').html("")
    char_index = 1
    for c,i in this.locale.characters
      continue if c == " "
      char = "<span id=\"char_#{char_index}\" class=\"char alpha_#{c.toLowerCase()}\">#{c}</span>" # title=\"#{char_index}\"
      $('#chars').append(char)
      char_index++
    return

  setClockSize: ->
    base = Math.min($(window).height(), $(window).width())
    $('#body_inner').css('width', $(window).width()).css('height', $(window).height())
    clock_size = Math.round(base * this.config.clock_size)
    clock_margin_top = Math.round(base * (1-this.config.clock_size) / 2)
    $('#clock_wrapper,#clock').css('width',clock_size).css('height',clock_size)
    $('#clock_wrapper').css('margin-top',clock_margin_top)

    clock_padding = Math.round(clock_size * this.config.clock_padding)
    $('#clock #chars').css('padding', clock_padding)
    char_height = Math.round(clock_size * (1-this.config.clock_padding*2) / 10)
    char_font_size = Math.round(char_height * this.config.clock_char_size)
    $('#clock #chars .char').css('height',char_height).css('line-height', "#{char_height}px").css('font-size',char_font_size)

    if this.config.clock_minutes
      minute_size = Math.round(clock_padding * this.config.clock_minute_size)
      minute_padding = Math.round(clock_padding * (1-this.config.clock_minute_size) / 2)
      $('.minute').css('width', minute_size).css('height', minute_size)
      $('#minute1').css('left',minute_padding).css('top', minute_padding)
      $('#minute2').css('right',minute_padding).css('top', minute_padding)
      $('#minute3').css('right',minute_padding).css('bottom', minute_padding)
      $('#minute4').css('left',minute_padding).css('bottom', minute_padding)
    else
      $('.minute').remove()

    return

  startTimer: ->
    this.updateTime()
    interval = this.config.clock_interval
    interval = this.config.demo_interval if this.config.mode == "demo"
    setInterval (->
      qlock.updateTime()
      return
    ), interval

    return

  updateTime: ->
        
    # set time
    if this.config.mode == "demo"
      if this.last_moment
        step = this.config.demo_step
        step = this.helperRandomInt(1,1440) if typeof this.config.demo_step is 'string' # must be random
        this.last_moment = moment(this.last_moment).add(step, 'minutes')
      else
        this.last_moment = moment()
      now = this.last_moment
    else
      now = moment()
      now = moment(now) #.add(32, 'minutes').add(7,'hours')

    # get time parts
    m = parseInt(now.format("m"))
    h = parseInt(now.format("h"))
    this.log "hour: #{h} / minute: #{m}"

    # stop if no change
    return if h == this.last_h && m == this.last_m

    # reset overwritten title time
    $('#clock').removeData('title-time')

    # set minutes if activated
    this.setMinutes(m, now) if this.config.clock_minutes

    this.last_m = m
    this.last_h = h
    this.new_chars = []
    
    # queue new words and activate them
    this.locale.setChars(this.locale.words, h, m, now)
    this.activateQueuedCharacters()
    
    return

  setMinutes: (m, now) ->
    mod_minute = m % 5
    $('.minute').removeClass('on')
    for x in [1..4]
      $("#minute#{x}").addClass('on') if mod_minute >= x
    return

  enqueueCharacters: (chars) ->
    this.new_chars.push chars.slice()
    return

  activateQueuedCharacters: ->
    this.log "activateQueuedCharacters"
    return if this.getQueueSignature(this.old_chars) == this.getQueueSignature(this.new_chars)

    this.log "change time!"

    for x,y in this.old_chars
      for z in x
        $("#char_#{z}").removeClass('on')

    for x,y in this.new_chars
      for z in x
        $("#char_#{z}").addClass('on')

    this.setTitle() if this.config.clock_titletime

    # time change event

    this.old_chars = this.new_chars.slice()

    return

  getQueueSignature: (queue) ->
    res = ""
    for x,y in queue
      for z in x
        res += "#{z}|"
    return res

  setTitle: ->
    time = ""
    for x,y in this.new_chars
      last_char = -1
      for z in x
        time += " " if last_char != -1 && last_char < z-1
        last_char = z
        time += $("#char_#{z}").text()
      time += " "
    time = time.trim()
    time = $('#clock').data('title-time') if $('#clock').data('title-time')
    document.title = time
    return

  activateAllWords: ->
    for key, value of this.locale.words
      this.enqueueCharacters value
    this.activateQueuedCharacters()
    return
  addLocale: (locale) ->
    this.locales.push locale
  
  helperNextHour: (h) ->
    return 1 if h == 12
    return h+1

  helperRandomInt: (min, max) ->
    Math.floor(Math.random() * (max - min + 1)) + min
  