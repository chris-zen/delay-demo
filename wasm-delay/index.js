const wasm = import("./pkg/wasm_delay");

var context;
var delay;
let playing = false;

function initAudioContext() {
  if (context === undefined) {
    console.log("Initialising audio ...");
    return navigator.mediaDevices
      .getUserMedia({audio : true, video : false})
      .then(stream => {
        console.log("Got an stream ...");
        context = new AudioContext();
        const mic = context.createMediaStreamSource(stream);
        const delayProcessor = context.createScriptProcessor(256, 2, 2);

        delayProcessor.onaudioprocess = function(event) {
          const input = event.inputBuffer;
          const output = event.outputBuffer;

          delay.process(
            input.getChannelData(0), input.getChannelData(1),
            output.getChannelData(0), output.getChannelData(1));
        }

        mic.connect(delayProcessor);
        delayProcessor.connect(context.destination);
        context.suspend();
        return context;
      })
      .catch(function(error) {
        console.log("Unable to access microphone");
        console.log(error);
      });
  } else {
    return Promise.resolve(context);
  }
}

window.onload = function() {
  wasm.then(wasm => {
    initAudioContext().then(context => {
      delay = new wasm.Delay(context.sampleRate);

      const play = document.getElementById('play');
      play.addEventListener('click', function() {
        console.log(context);
        if (playing) {
          console.log("Suspending audio");
          context.suspend();
          play.innerHTML = "Start"
          play.className = "btn btn-outline-success btn-block";
        } else {
          console.log("Resuming audio");
          context.resume();
          play.innerHTML = "Stop";
          play.className = "btn btn-outline-danger btn-block";
        }
        playing = !playing;
      });

      const delaySeconds = delay.get_delay_seconds();
      const delayValue = document.getElementById('delay_value');
      delayValue.value = delaySeconds + " seconds";

      const delaySlider = document.getElementById('delay');
      delaySlider.value = delaySeconds * 1000;
      delaySlider.addEventListener('input', function() {
        delay.set_delay_seconds(delaySlider.value / 1000.0);
        delayValue.value = delay.get_delay_seconds() + " seconds";
      });

      const feedback = delay.get_feedback() * 100;
      const feedbackValue = document.getElementById('feedback_value');
      feedbackValue.value = feedback + " %";

      const feedbackSlider = document.getElementById('feedback');
      feedbackSlider.value = feedback;
      feedbackSlider.addEventListener('input', function() {
        delay.set_feedback(feedbackSlider.value / 100.0);
        feedbackValue.value = feedbackSlider.value + " %";
      });

      const wetDry = delay.get_wet_dry_ratio() * 100;
      const wetDryValue = document.getElementById('wet_dry_value');
      wetDryValue.value = wetDry + " %";

      const wetDrySlider = document.getElementById('wet_dry');
      wetDrySlider.value = wetDry;
      wetDrySlider.addEventListener('input', function() {
        delay.set_wet_dry_ratio(wetDrySlider.value / 100.0);
        wetDryValue.value = wetDrySlider.value + " %";
      });
    });
  });
}
