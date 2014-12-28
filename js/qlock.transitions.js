(function() {
  var transition;

  transition = {
    name: "explode",
    method: function(all_old_chars, all_new_chars, old_chars, new_chars) {
      var jquery_all_new_chars, jquery_all_old_chars, jquery_new_chars, jquery_old_chars;
      qlock.log(old_chars);
      qlock.log(new_chars);
      jquery_all_old_chars = qlock.helper.transition.buildElements(all_old_chars);
      jquery_all_new_chars = qlock.helper.transition.buildElements(all_new_chars);
      jquery_old_chars = qlock.helper.transition.buildElements(old_chars);
      jquery_new_chars = qlock.helper.transition.buildElements(new_chars);
      jquery_all_old_chars.velocity({
        colorAlpha: 0.14,
        complete: function(elements) {}
      });
      jquery_all_new_chars.velocity({
        colorAlpha: 1,
        complete: function(elements) {}
      });
    }
  };

  qlock.addTransition(transition);

}).call(this);
