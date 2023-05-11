function(event, {set_port_value}) {
  function handle_event(symbol, value) {
    const time_link = event.icon.find("[mod-port-symbol=time_link]");
    const is_time_linked = time_link.hasClass("active");
    
    switch (symbol) {
        case "time_link":
            if(value == 1) {
              time_link.addClass("active");
            } else {
              time_link.removeClass("active");
            }
            break;
        case "time_left":
            if(is_time_linked) {
              set_port_value("time_right", value);
            }
            break;
        case "time_right":
            if(is_time_linked) {
              set_port_value("time_left", value);
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