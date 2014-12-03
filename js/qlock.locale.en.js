(function() {
  var locale;

  locale = {
    code: "en",
    name: "english",
    characters: "ITLISASTIME ACQUARTEROC TWENTYFIVEX HALFBTENFTO PASTERUNINE ONESIXTHREE FOURFIVETWO EIGHTELEVEN SEVENTWELVE TENSEOCLOCK",
    words: {
      intro: [1, 2, 4, 5],
      half: [34, 35, 36, 37],
      past: [45, 46, 47, 48],
      hour: [105, 106, 107, 108, 109, 110],
      to: [43, 44],
      minutes_5: [29, 30, 31, 32],
      minutes_10: [39, 40, 41],
      minutes_15: [14, 15, 16, 17, 18, 19, 20],
      minutes_20: [23, 24, 25, 26, 27, 28],
      minutes_25: [23, 24, 25, 26, 27, 28, 29, 30, 31, 32],
      hours_1: [56, 57, 58],
      hours_2: [75, 76, 77],
      hours_3: [62, 63, 64, 65, 66],
      hours_4: [67, 68, 69, 70],
      hours_5: [71, 72, 73, 74],
      hours_6: [59, 60, 61],
      hours_7: [89, 90, 91, 92, 93],
      hours_8: [78, 79, 80, 81, 82],
      hours_9: [52, 53, 54, 55],
      hours_10: [100, 101, 102],
      hours_11: [83, 84, 85, 86, 87, 88],
      hours_12: [94, 95, 96, 97, 98, 99]
    },
    setChars: function(h, m, now) {
      if (qlock.config.clock_intro) {
        qlock.enqueueCharacters(this.words.intro);
      }
      if (m >= 0 && m < 5) {
        qlock.enqueueCharacters(this.words["hours_" + h]);
        qlock.enqueueCharacters(this.words.hour);
      } else if (m >= 5 && m < 10) {
        qlock.enqueueCharacters(this.words.minutes_5);
        qlock.enqueueCharacters(this.words.past);
        qlock.enqueueCharacters(this.words["hours_" + h]);
      } else if (m >= 10 && m < 15) {
        qlock.enqueueCharacters(this.words.minutes_10);
        qlock.enqueueCharacters(this.words.past);
        qlock.enqueueCharacters(this.words["hours_" + h]);
      } else if (m >= 15 && m < 20) {
        qlock.enqueueCharacters(this.words.minutes_15);
        qlock.enqueueCharacters(this.words.past);
        qlock.enqueueCharacters(this.words["hours_" + h]);
      } else if (m >= 20 && m < 25) {
        qlock.enqueueCharacters(this.words.minutes_20);
        qlock.enqueueCharacters(this.words.past);
        qlock.enqueueCharacters(this.words["hours_" + h]);
      } else if (m >= 25 && m < 30) {
        qlock.enqueueCharacters(this.words.minutes_25);
        qlock.enqueueCharacters(this.words.past);
        qlock.enqueueCharacters(this.words["hours_" + h]);
      } else if (m >= 30 && m < 35) {
        qlock.enqueueCharacters(this.words.half);
        qlock.enqueueCharacters(this.words.past);
        qlock.enqueueCharacters(this.words["hours_" + h]);
      } else if (m >= 35 && m < 40) {
        qlock.enqueueCharacters(this.words.minutes_25);
        qlock.enqueueCharacters(this.words.to);
        qlock.enqueueCharacters(this.words["hours_" + (qlock.helper.nextHour(h))]);
      } else if (m >= 40 && m < 45) {
        qlock.enqueueCharacters(this.words.minutes_20);
        qlock.enqueueCharacters(this.words.to);
        qlock.enqueueCharacters(this.words["hours_" + (qlock.helper.nextHour(h))]);
      } else if (m >= 45 && m < 50) {
        qlock.enqueueCharacters(this.words.minutes_15);
        qlock.enqueueCharacters(this.words.to);
        qlock.enqueueCharacters(this.words["hours_" + (qlock.helper.nextHour(h))]);
      } else if (m >= 50 && m < 55) {
        qlock.enqueueCharacters(this.words.minutes_10);
        qlock.enqueueCharacters(this.words.to);
        qlock.enqueueCharacters(this.words["hours_" + (qlock.helper.nextHour(h))]);
      } else if (m >= 55) {
        qlock.enqueueCharacters(this.words.minutes_5);
        qlock.enqueueCharacters(this.words.to);
        qlock.enqueueCharacters(this.words["hours_" + (qlock.helper.nextHour(h))]);
      }
    }
  };

  qlock.addLocale(locale);

}).call(this);
