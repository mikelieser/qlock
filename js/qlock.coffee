qlock = 

  new_chars: [] # queue for new active chars
  old_chars: [] # queue for old active chars
  last_m: -1 # marker last minute
  last_h: -1 # marker last hour
  last_moment: null # last moment (demo mode)
  config: null # merged user params with defaults
  locales: [] # array of locales
  locale: null # active locale
  
  # config with default params
  config:
    mode: "standard" # standard, all_words, demo
    clock_interval: 1000 # ms, 1000 is fine, increase for better performance and less efficiency
    demo_interval: 3000 # ms, if mode is "demo"
    demo_step: "random" # in minutes (1, 5, 60 or "random"), if mode is "demo"
    debug: false # show debug messages?
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
    
    clock_rows: 10 # default scheme
    clock_cols: 11 # default scheme
  
  log: (msg) ->
    console.log(msg) if @config.debug
    return
  
  init: (params) ->
    
    # merge defaults with user params
    @config = $.extend @config, params
    @log @config
    
    # set locale
    for l in @locales
      if l.code == @config.locale
        @locale = l
        break
    
    unless @locale
      @log "locale #{@config.locale} not found"
      return
      
    @log "locale: #{@locale.code} (#{@locale.name})"
    
    # disable css transitions on page load
    $('body').addClass('no-transitions')

    @fillClockCharacters()
    @setClockSize()
    theme = @config.theme # one theme
    theme = @config.theme[@helper.randomInt(0, theme.length-1)] if typeof @config.theme is 'object' # array of themes, get one by random
    $('body').addClass("theme-#{theme}")
    $('#clock_wrapper').addClass("animated #{@config.clock_start_animation}") if @config.clock_start_animation != "none"
    
    if @config.mode == "all_words"
      @activateAllWords() # show all used chars
    else
      @startTimer()

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
    for c,i in @locale.characters
      if c == " "
        $('#chars').append('<br />')
      else
        $('#chars').append("<span id=\"char_#{char_index}\" class=\"char alpha_#{c.toLowerCase()}\">#{c}</span>") # title=\"#{char_index}\"
        char_index++
        
    return

  setCharRowsAndCols: ->
    
    cols = 0
    rows = 1
    tmp_cols = 0
    
    for c,i in @locale.characters
      if c == " "
        rows++
        if tmp_cols > cols
          cols = tmp_cols
        tmp_cols = 0
      else
        tmp_cols++
    
    cols = tmp_cols if tmp_cols > cols
    
    @log "rows: #{rows}"
    @log "cols: #{cols}"
    
    @config.clock_rows = rows
    @config.clock_cols = cols
    
    return
    
  setClockSize: ->
    
    @setCharRowsAndCols()

    base = Math.min($(window).height(), $(window).width())
    $('#body_inner').css('width', $(window).width()).css('height', $(window).height())
    clock_size = Math.round(base * @config.clock_size)
    clock_margin_top = Math.round(base * (1-@config.clock_size) / 2)
    $('#clock_wrapper,#clock').css('width',clock_size).css('height',clock_size)
    $('#clock_wrapper').css('margin-top',clock_margin_top)

    clock_padding = Math.round(clock_size * @config.clock_padding)
    $('#clock #chars').css('padding', clock_padding)
    char_height = Math.round(clock_size * (1-@config.clock_padding*2) / @config.clock_rows)
    char_width = Math.floor(clock_size * (1-@config.clock_padding*2) / @config.clock_cols)
    
    #@log char_height
    #@log char_width
    
    char_font_size = Math.round(char_height * @config.clock_char_size)
    $('#clock #chars .char').css('height',char_height).css('width',char_width).css('line-height', "#{char_height}px").css('font-size',char_font_size)

    if @config.clock_minutes
      minute_size = Math.round(clock_padding * @config.clock_minute_size)
      minute_padding = Math.round(clock_padding * (1-@config.clock_minute_size) / 2)
      $('.minute').css('width', minute_size).css('height', minute_size)
      $('#minute1').css('left',minute_padding).css('top', minute_padding)
      $('#minute2').css('right',minute_padding).css('top', minute_padding)
      $('#minute3').css('right',minute_padding).css('bottom', minute_padding)
      $('#minute4').css('left',minute_padding).css('bottom', minute_padding)
    else
      $('.minute').remove()

    return

  startTimer: ->
    @updateTime()
    interval = @config.clock_interval
    interval = @config.demo_interval if @config.mode == "demo"
    setInterval (->
      qlock.updateTime()
      return
    ), interval

    return

  updateTime: ->
        
    # set time
    if @config.mode == "demo"
      if @last_moment
        step = @config.demo_step
        step = @helper.randomInt(1,1440) if typeof @config.demo_step is 'string' # must be random
        @last_moment = moment(@last_moment).add(step, 'minutes')
      else
        @last_moment = moment()
      now = @last_moment
    else
      now = moment()
      now = moment(now) #.add(32, 'minutes').add(7,'hours')

    # get time parts
    m = parseInt(now.format("m"))
    h = parseInt(now.format("h"))
    @log "hour: #{h} / minute: #{m}"

    # set minutes if activated
    @setMinutes(m, now) if @config.clock_minutes

    @last_m = m
    @last_h = h
    @new_chars = []
    
    # queue new words and activate them
    @locale.setChars(@locale.words, h, m, now)
    @activateQueuedCharacters()
    
    return

  setMinutes: (m, now) ->
    mod_minute = m % 5
    $('.minute').removeClass('on')
    for x in [1..4]
      $("#minute#{x}").addClass('on') if mod_minute >= x
    return

  enqueueCharacters: (chars) ->
    @new_chars.push chars.slice()
    return

  activateQueuedCharacters: ->
    @log "activateQueuedCharacters"
    return if @getQueueSignature(@old_chars) == @getQueueSignature(@new_chars)

    @log "change time!"

    for x,y in @old_chars
      for z in x
        $("#char_#{z}").removeClass('on')

    for x,y in @new_chars
      for z in x
        $("#char_#{z}").addClass('on')

    @setDocumentTitle() if @config.clock_titletime

    # time change event

    @old_chars = @new_chars.slice()

    return

  getQueueSignature: (queue) ->
    res = ""
    for x,y in queue
      for z in x
        res += "#{z}|"
    return res

  setTitle: (title) ->
    $('#clock').data('title-time', title)
  setDocumentTitle: ->
    time = ""
    for x,y in @new_chars
      last_char = -1
      for z in x
        time += " " if last_char != -1 && last_char < z-1
        last_char = z
        time += $("#char_#{z}").text()
      time += " "
    time = time.trim()
    
    if $('#clock').data('title-time')
      time = $('#clock').data('title-time') 
      $('#clock').removeData('title-time')
      
    document.title = time
    
    return

  activateAllWords: ->
    for key, value of @locale.words
      @enqueueCharacters value
    @activateQueuedCharacters()
    return
  addLocale: (locale) ->
    @locales.push locale
  
  
  helper:
    nextHour: (h) ->
      return 1 if h == 12
      return h+1

    randomInt: (min, max) ->
      Math.floor(Math.random() * (max - min + 1)) + min
  