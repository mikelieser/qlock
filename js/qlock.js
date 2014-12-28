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
  transitions: [],
  transition: null,
  config: {
    mode: "standard",
    clock_interval: 1000,
    demo_interval: 3000,
    demo_step: "random",
    debug: false,
    locale: "en",
    theme: ["standard"],
    transition: "none",
    clock_intro: true,
    clock_size: 0.8,
    clock_padding: 0.14,
    clock_char_size: 0.65,
    clock_minutes: true,
    clock_minute_size: 0.4,
    clock_titletime: true,
    clock_start_animation: "none",
    clock_rows: 10,
    clock_cols: 11
  },
  log: function(msg) {
    if (this.config.debug) {
      console.log(msg);
    }
  },
  init: function(params) {
    var l, t, theme, _i, _j, _len, _len1, _ref, _ref1;
    this.config = $.extend(this.config, params);
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
    _ref1 = this.transitions;
    for (_j = 0, _len1 = _ref1.length; _j < _len1; _j++) {
      t = _ref1[_j];
      if (t.name === this.config.transition) {
        this.transition = t;
        break;
      }
    }
    $('body').addClass('no-transitions');
    this.fillClockCharacters();
    this.setClockSize();
    theme = this.config.theme;
    if (typeof this.config.theme === 'object') {
      theme = this.config.theme[this.helper.randomInt(0, theme.length - 1)];
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
    var c, char_index, i, row, _i, _len, _ref;
    $('#chars').html("");
    char_index = 1;
    row = 1;
    _ref = this.locale.characters;
    for (i = _i = 0, _len = _ref.length; _i < _len; i = ++_i) {
      c = _ref[i];
      if ($("#chars #row" + row).length === 0) {
        $("#chars").append("<span id=\"row" + row + "\" class=\"row\"></span>");
      }
      if (c === " ") {
        $("#chars #row" + row).append('<br />');
        row++;
      } else {
        $("#chars #row" + row).append("<span id=\"char_" + char_index + "\" class=\"char alpha_" + (c.toLowerCase()) + "\">" + c + "</span>");
        char_index++;
      }
    }
  },
  setCharRowsAndCols: function() {
    var c, cols, i, rows, tmp_cols, _i, _len, _ref;
    cols = 0;
    rows = 1;
    tmp_cols = 0;
    _ref = this.locale.characters;
    for (i = _i = 0, _len = _ref.length; _i < _len; i = ++_i) {
      c = _ref[i];
      if (c === " ") {
        rows++;
        if (tmp_cols > cols) {
          cols = tmp_cols;
        }
        tmp_cols = 0;
      } else {
        tmp_cols++;
      }
    }
    if (tmp_cols > cols) {
      cols = tmp_cols;
    }
    this.log("rows: " + rows);
    this.log("cols: " + cols);
    this.config.clock_rows = rows;
    this.config.clock_cols = cols;
  },
  setClockSize: function() {
    var base, char_font_size, char_height, char_width, clock_margin_top, clock_padding, clock_size, minute_padding, minute_size;
    this.setCharRowsAndCols();
    base = Math.min($(window).height(), $(window).width());
    $('#body_inner').css('width', $(window).width()).css('height', $(window).height());
    clock_size = Math.round(base * this.config.clock_size);
    clock_margin_top = Math.round(base * (1 - this.config.clock_size) / 2);
    $('#clock_wrapper,#clock').css('width', clock_size).css('height', clock_size);
    $('#clock_wrapper').css('margin-top', clock_margin_top);
    clock_padding = Math.round(clock_size * this.config.clock_padding);
    $('#clock #chars').css('padding', clock_padding);
    char_height = Math.round(clock_size * (1 - this.config.clock_padding * 2) / this.config.clock_rows);
    char_width = Math.floor(clock_size * (1 - this.config.clock_padding * 2) / this.config.clock_cols);
    char_font_size = Math.round(char_height * this.config.clock_char_size);
    $('#clock #chars .char').css('height', char_height).css('width', char_width).css('line-height', "" + char_height + "px").css('font-size', char_font_size);
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
          step = this.helper.randomInt(1, 1440);
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
    if (this.config.clock_minutes) {
      this.setMinutes(m, now);
    }
    this.last_m = m;
    this.last_h = h;
    this.new_chars = [];
    this.locale.setChars(h, m, now);
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
    this.log("activateQueuedCharacters");
    if (this.getQueueSignature(this.old_chars) === this.getQueueSignature(this.new_chars)) {
      return;
    }
    this.log("change time!");
    if (this.config.transition === "none") {
      this.helper.transition.buildElements(this.old_chars).removeClass('on');
      this.helper.transition.buildElements(this.new_chars).addClass('on');
    } else {
      this.transition.method(this.old_chars, this.new_chars, this.helper.transition.calculateOldChars(), this.helper.transition.calculateNewChars());
    }
    if (this.config.clock_titletime) {
      this.setDocumentTitle();
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
  setTitle: function(title) {
    return $('#clock').data('title-time', title);
  },
  setDocumentTitle: function() {
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
      $('#clock').removeData('title-time');
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
  addTransition: function(transition) {
    return this.transitions.push(transition);
  },
  helper: {
    nextHour: function(h) {
      if (h === 12) {
        return 1;
      }
      return h + 1;
    },
    randomInt: function(min, max) {
      return Math.floor(Math.random() * (max - min + 1)) + min;
    },
    transition: {
      buildElements: function(array) {
        var counter, str, x, y, z, _i, _j, _len, _len1;
        str = "";
        counter = 0;
        for (y = _i = 0, _len = array.length; _i < _len; y = ++_i) {
          x = array[y];
          for (_j = 0, _len1 = x.length; _j < _len1; _j++) {
            z = x[_j];
            if (counter > 0) {
              str += ",";
            }
            str += "#char_" + z;
            counter++;
          }
        }
        return $(str);
      },
      calculateOldChars: function() {
        return qlock.old_chars;
      },
      calculateNewChars: function() {
        return qlock.new_chars;
      }
    }
  }
};
