(function() {
  var locale;

  locale = {
    code: "ticktack",
    name: "ticktack",
    characters: "ES IST TICK TACK UHRZEIT",
    words: {
      tick: [6, 7, 8, 9],
      tack: [10, 11, 12, 13]
    },
    setChars: function(words, h, m, now) {
      var s;
      s = parseInt(now.format("s"));
      if (s % 2 === 0) {
        $('.minute').addClass('on');
        qlock.enqueueCharacters(words.tick);
        qlock.setTitle("TICK");
      } else {
        $('.minute').removeClass('on');
        qlock.enqueueCharacters(words.tack);
        qlock.setTitle("TACK");
      }
    }
  };

  qlock.addLocale(locale);

}).call(this);
