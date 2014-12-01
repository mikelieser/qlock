var qlock;

qlock = {
  new_chars: [],
  old_chars: [],
  last_m: -1,
  last_h: -1,
  last_moment: null,
  config: null,
  locales: [],
  locale: null,
  defaults: {
    mode: "standard",
    clock_interval: 1000,
    demo_interval: 4000,
    demo_step: "random",
    debug: false,
    locale: "en",
    theme: ["standard"],
    clock_intro: true,
    clock_size: 0.8,
    clock_padding: 0.14,
    clock_char_size: 0.65,
    clock_minutes: true,
    clock_minute_size: 0.4,
    clock_titletime: true,
    clock_start_animation: "zoomIn"
  },
  log: function(msg) {
    if (this.config.debug) {
      console.log(msg);
    }
  },
  init: function(params) {
    var l, theme, _i, _len, _ref;
    this.config = $.extend(this.defaults, params);
    this.log(this.config);
    _ref = this.locales;
    for (_i = 0, _len = _ref.length; _i < _len; _i++) {
      l = _ref[_i];
      if (l.code === this.config.locale) {
        this.locale = l;
        break;
      }
    }
    if (!this.locale) {
      this.log("locale " + this.config.locale + " not found");
      return;
    }
    this.log("locale: " + this.locale.code + " (" + this.locale.name + ")");
    $('body').addClass('no-transitions');
    this.fillClockCharacters();
    this.setClockSize();
    theme = this.config.theme;
    if (typeof this.config.theme === 'object') {
      theme = this.config.theme[this.helperRandomInt(0, theme.length - 1)];
    }
    $('body').addClass("theme-" + theme);
    if (this.config.clock_start_animation !== "none") {
      $('#clock_wrapper').addClass("animated " + this.config.clock_start_animation);
    }
    if (this.config.mode === "all_words") {
      this.activateAllWords();
    } else {
      this.startTimer();
    }
    $(window).load(function() {
      $('body').removeClass('no-transitions');
    });
    $(window).resize(function() {
      qlock.setClockSize();
    });
  },
  fillClockCharacters: function() {
    var c, char, char_index, i, _i, _len, _ref;
    $('#chars').html("");
    char_index = 1;
    _ref = this.locale.characters;
    for (i = _i = 0, _len = _ref.length; _i < _len; i = ++_i) {
      c = _ref[i];
      if (c === " ") {
        continue;
      }
      char = "<span id=\"char_" + char_index + "\" class=\"char alpha_" + (c.toLowerCase()) + "\">" + c + "</span>";
      $('#chars').append(char);
      char_index++;
    }
  },
  setClockSize: function() {
    var base, char_font_size, char_height, clock_margin_top, clock_padding, clock_size, minute_padding, minute_size;
    base = Math.min($(window).height(), $(window).width());
    $('#body_inner').css('width', $(window).width()).css('height', $(window).height());
    clock_size = Math.round(base * this.config.clock_size);
    clock_margin_top = Math.round(base * (1 - this.config.clock_size) / 2);
    $('#clock_wrapper,#clock').css('width', clock_size).css('height', clock_size);
    $('#clock_wrapper').css('margin-top', clock_margin_top);
    clock_padding = Math.round(clock_size * this.config.clock_padding);
    $('#clock #chars').css('padding', clock_padding);
    char_height = Math.round(clock_size * (1 - this.config.clock_padding * 2) / 10);
    char_font_size = Math.round(char_height * this.config.clock_char_size);
    $('#clock #chars .char').css('height', char_height).css('line-height', "" + char_height + "px").css('font-size', char_font_size);
    if (this.config.clock_minutes) {
      minute_size = Math.round(clock_padding * this.config.clock_minute_size);
      minute_padding = Math.round(clock_padding * (1 - this.config.clock_minute_size) / 2);
      $('.minute').css('width', minute_size).css('height', minute_size);
      $('#minute1').css('left', minute_padding).css('top', minute_padding);
      $('#minute2').css('right', minute_padding).css('top', minute_padding);
      $('#minute3').css('right', minute_padding).css('bottom', minute_padding);
      $('#minute4').css('left', minute_padding).css('bottom', minute_padding);
    } else {
      $('.minute').remove();
    }
  },
  startTimer: function() {
    var interval;
    this.updateTime();
    interval = this.config.clock_interval;
    if (this.config.mode === "demo") {
      interval = this.config.demo_interval;
    }
    setInterval((function() {
      qlock.updateTime();
    }), interval);
  },
  updateTime: function() {
    var h, m, now, step;
    if (this.config.mode === "demo") {
      if (this.last_moment) {
        step = this.config.demo_step;
        if (typeof this.config.demo_step === 'string') {
          step = this.helperRandomInt(1, 1440);
        }
        this.last_moment = moment(this.last_moment).add(step, 'minutes');
      } else {
        this.last_moment = moment();
      }
      now = this.last_moment;
    } else {
      now = moment();
      now = moment(now);
    }
    m = parseInt(now.format("m"));
    h = parseInt(now.format("h"));
    this.log("hour: " + h + " / minute: " + m);
    if (h === this.last_h && m === this.last_m) {
      return;
    }
    $('#clock').removeData('title-time');
    if (this.config.clock_minutes) {
      this.setMinutes(m, now);
    }
    this.last_m = m;
    this.last_h = h;
    this.new_chars = [];
    this.locale.setChars(this.locale.words, h, m, now);
    this.activateQueuedCharacters();
  },
  setMinutes: function(m, now) {
    var mod_minute, x, _i;
    mod_minute = m % 5;
    $('.minute').removeClass('on');
    for (x = _i = 1; _i <= 4; x = ++_i) {
      if (mod_minute >= x) {
        $("#minute" + x).addClass('on');
      }
    }
  },
  enqueueCharacters: function(chars) {
    this.new_chars.push(chars.slice());
  },
  activateQueuedCharacters: function() {
    var x, y, z, _i, _j, _k, _l, _len, _len1, _len2, _len3, _ref, _ref1;
    this.log("activateQueuedCharacters");
    if (this.getQueueSignature(this.old_chars) === this.getQueueSignature(this.new_chars)) {
      return;
    }
    this.log("change time!");
    _ref = this.old_chars;
    for (y = _i = 0, _len = _ref.length; _i < _len; y = ++_i) {
      x = _ref[y];
      for (_j = 0, _len1 = x.length; _j < _len1; _j++) {
        z = x[_j];
        $("#char_" + z).removeClass('on');
      }
    }
    _ref1 = this.new_chars;
    for (y = _k = 0, _len2 = _ref1.length; _k < _len2; y = ++_k) {
      x = _ref1[y];
      for (_l = 0, _len3 = x.length; _l < _len3; _l++) {
        z = x[_l];
        $("#char_" + z).addClass('on');
      }
    }
    if (this.config.clock_titletime) {
      this.showTitleTime();
    }
    this.old_chars = this.new_chars.slice();
  },
  getQueueSignature: function(queue) {
    var res, x, y, z, _i, _j, _len, _len1;
    res = "";
    for (y = _i = 0, _len = queue.length; _i < _len; y = ++_i) {
      x = queue[y];
      for (_j = 0, _len1 = x.length; _j < _len1; _j++) {
        z = x[_j];
        res += "" + z + "|";
      }
    }
    return res;
  },
  showTitleTime: function() {
    var last_char, time, x, y, z, _i, _j, _len, _len1, _ref;
    time = "";
    _ref = this.new_chars;
    for (y = _i = 0, _len = _ref.length; _i < _len; y = ++_i) {
      x = _ref[y];
      last_char = -1;
      for (_j = 0, _len1 = x.length; _j < _len1; _j++) {
        z = x[_j];
        if (last_char !== -1 && last_char < z - 1) {
          time += " ";
        }
        last_char = z;
        time += $("#char_" + z).text();
      }
      time += " ";
    }
    time = time.trim();
    if ($('#clock').data('title-time')) {
      time = $('#clock').data('title-time');
    }
    document.title = time;
  },
  activateAllWords: function() {
    var key, value, _ref;
    _ref = this.locale.words;
    for (key in _ref) {
      value = _ref[key];
      this.enqueueCharacters(value);
    }
    this.activateQueuedCharacters();
  },
  addLocale: function(locale) {
    return this.locales.push(locale);
  },
  helperNextHour: function(h) {
    if (h === 12) {
      return 1;
    }
    return h + 1;
  },
  helperRandomInt: function(min, max) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  }
};
