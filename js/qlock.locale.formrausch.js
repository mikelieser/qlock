(function() {
  var locale;

  locale = {
    code: "formrausch",
    name: "formrausch",
    characters: "ESRISTSFÜNF ZEHNZWANZIG DREIVIERTEL VøRFøRMNACH HALBRELFÜNF EINSAATZWEI DREIUSCVIER SECHSΣHACHT SIEBENZWøLF ZEHNEUNDUHR",
    words: {
      intro: [1, 2, 4, 5, 6],
      before: [34, 35, 36],
      past: [41, 42, 43, 44],
      half: [45, 46, 47, 48],
      hour: [108, 109, 110],
      minutes_5: [8, 9, 10, 11],
      minutes_10: [12, 13, 14, 15],
      minutes_15: [27, 28, 29, 30, 31, 32, 33],
      minutes_20: [16, 17, 18, 19, 20, 21, 22],
      minutes_45: [23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33],
      hours_1: [56, 57, 58, 59],
      hours_1s: [56, 57, 58],
      hours_2: [63, 64, 65, 66],
      hours_3: [67, 68, 69, 70],
      hours_4: [74, 75, 76, 77],
      hours_5: [52, 53, 54, 55],
      hours_6: [78, 79, 80, 81, 82],
      hours_7: [89, 90, 91, 92, 93, 94],
      hours_8: [85, 86, 87, 88],
      hours_9: [103, 104, 105, 106],
      hours_10: [100, 101, 102, 103],
      hours_11: [50, 51, 52],
      hours_12: [95, 96, 97, 98, 99],
      feierabend: [11, 25, 26, 29, 30, 61, 92, 93, 106, 107],
      formrausch: [37, 38, 39, 40, 49, 60, 71, 72, 73, 84],
      happa: [27, 28, 29, 38, 39, 40, 49, 50, 51, 60, 61, 62, 71, 72, 73, 82, 83, 84, 6, 105, 25, 47, 69, 86, 64, 42]
    },
    setChars: function(words, h, m, now) {
      var chars_hours, hh;
      hh = parseInt(now.format("H"));
      if (hh === 11 && m >= 15 && m <= 45 && m % 2 === 1) {
        $('.minute').addClass('on');
        qlock.enqueueCharacters(words.happa);
        $('#clock').data('title-time', 'FRÜHSTÜCKSZEIT ;)');
        return;
      }
      if (hh === 18) {
        $('.minute').addClass('on');
        $('#clock').data('title-time', 'FEIERABEND ;)');
        if (m % 2 === 0) {
          qlock.enqueueCharacters(words.intro);
          qlock.enqueueCharacters(words.feierabend);
        } else {
          qlock.enqueueCharacters(words.formrausch);
        }
        return;
      }
      if (qlock.config.clock_intro) {
        qlock.enqueueCharacters(words.intro);
      }
      if (m >= 0 && m < 5) {
        chars_hours = words["hours_" + h];
        if (h === 1) {
          chars_hours = words["hours_1s"];
        }
        qlock.enqueueCharacters(chars_hours);
        qlock.enqueueCharacters(words.hour);
      } else if (m >= 5 && m < 10) {
        qlock.enqueueCharacters(words.minutes_5);
        qlock.enqueueCharacters(words.past);
        qlock.enqueueCharacters(words["hours_" + h]);
      } else if (m >= 10 && m < 15) {
        qlock.enqueueCharacters(words.minutes_10);
        qlock.enqueueCharacters(words.past);
        qlock.enqueueCharacters(words["hours_" + h]);
      } else if (m >= 15 && m < 20) {
        qlock.enqueueCharacters(words.minutes_15);
        qlock.enqueueCharacters(words.past);
        qlock.enqueueCharacters(words["hours_" + h]);
      } else if (m >= 20 && m < 25) {
        qlock.enqueueCharacters(words.minutes_20);
        qlock.enqueueCharacters(words.past);
        qlock.enqueueCharacters(words["hours_" + h]);
      } else if (m >= 25 && m < 30) {
        qlock.enqueueCharacters(words.minutes_5);
        qlock.enqueueCharacters(words.before);
        qlock.enqueueCharacters(words.half);
        qlock.enqueueCharacters(words["hours_" + (qlock.helper.nextHour(h))]);
      } else if (m >= 30 && m < 35) {
        qlock.enqueueCharacters(words.half);
        qlock.enqueueCharacters(words["hours_" + (qlock.helper.nextHour(h))]);
      } else if (m >= 35 && m < 40) {
        qlock.enqueueCharacters(words.minutes_5);
        qlock.enqueueCharacters(words.past);
        qlock.enqueueCharacters(words.half);
        qlock.enqueueCharacters(words["hours_" + (qlock.helper.nextHour(h))]);
      } else if (m >= 40 && m < 45) {
        qlock.enqueueCharacters(words.minutes_20);
        qlock.enqueueCharacters(words.before);
        qlock.enqueueCharacters(words["hours_" + (qlock.helper.nextHour(h))]);
      } else if (m >= 45 && m < 50) {
        qlock.enqueueCharacters(words.minutes_15);
        qlock.enqueueCharacters(words.before);
        qlock.enqueueCharacters(words["hours_" + (qlock.helper.nextHour(h))]);
      } else if (m >= 50 && m < 55) {
        qlock.enqueueCharacters(words.minutes_10);
        qlock.enqueueCharacters(words.before);
        qlock.enqueueCharacters(words["hours_" + (qlock.helper.nextHour(h))]);
      } else if (m >= 55) {
        qlock.enqueueCharacters(words.minutes_5);
        qlock.enqueueCharacters(words.before);
        qlock.enqueueCharacters(words["hours_" + (qlock.helper.nextHour(h))]);
      }
    }
  };

  qlock.addLocale(locale);

}).call(this);
