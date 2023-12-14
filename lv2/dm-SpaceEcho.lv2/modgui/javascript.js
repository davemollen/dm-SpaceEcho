function(event) {
  function handle_event(symbol, value) {
    
    switch (symbol) {
      case "time_link":
            const time_link = event.icon.find("[mod-port-symbol=time_link]");
            const time_right = event.icon.find("[mod-port-symbol=time_right]");
            const synced_time_right = event.icon.find(".synced-time-knob");

            if(value == 1) {
              time_link.addClass("on");
              time_right.addClass("hide");
              synced_time_right.removeClass("hide");
            } else {
              time_link.removeClass("on");
              time_right.removeClass("hide");
              synced_time_right.addClass("hide");
            }
            break;
        case "limiter":
            const limiter = event.icon.find("[mod-port-symbol=limiter]");
            if(value == 1) {
              limiter.addClass("on");
            } else {
              limiter.removeClass("on");
            }
            break;
        case "hold":
            const hold = event.icon.find("[mod-port-symbol=hold]");
            if(value == 1) {
              hold.addClass("on");
            } else {
              hold.removeClass("on");
            }
            break;
        default:
            break;
    }
  }

  if (event.type == 'start') {
    const ports = event.ports;
    for (const p in ports) {
      handle_event(ports[p].symbol, ports[p].value);
    }
  }
  else if (event.type == 'change') {  
    handle_event(event.symbol, event.value);
  }
}