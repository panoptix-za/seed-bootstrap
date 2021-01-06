export function create_popper(toggle_element, popup_element, on_apply_styles) {
  return Popper.createPopper(toggle_element, popup_element, {
    strategy: "fixed",
    placement: 'bottom-start',
    modifiers: [
      {
        name: 'offset',
        options: {
          offset: [0, 2],
        },
      },
      {
        name: 'applyStyles',
        fn({ state }) {
          const popup_style =
            Object
              .entries(state.styles.popper)
              .map(([prop, value]) => `${prop}: ${value};`)
              .join(" ");

          on_apply_styles(popup_style);
        }
      },
    ],
  });
}

export function update_popper(popper_instance) {
  popper_instance.update();
}

export function destroy_popper(popper_instance) {
  popper_instance.destroy();
}
