(function() {
  var locale;

  locale = {
    code: "ticktack",
    name: "ticktack",
    characters: "TICK TACK",
    words: {
      tick: [1, 2, 3, 4],
      tack: [5, 6, 7, 8]
    },
    setChars: function(h, m, now) {
      var s;
      s = parseInt(now.format("s"));
      if (s % 2 === 0) {
        $('.minute').addClass('on');
        qlock.enqueueCharacters(this.words.tick);
        qlock.setTitle("TICK");
      } else {
        $('.minute').removeClass('on');
        qlock.enqueueCharacters(this.words.tack);
        qlock.setTitle("TACK");
      }
    }
  };

  qlock.addLocale(locale);

}).call(this);
