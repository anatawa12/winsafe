/// Implements `Send`, `Sync` and `Child` traits for a control.
macro_rules! impl_send_sync_debug_child {
	($name:ident) => {
		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "HWND {}, ID {}",
					self.hwnd(),
					self.ctrl_id(),
				)
			}
		}

		impl crate::gui::traits::Child for $name {
			fn hwnd_ref(&self) -> &crate::HWND {
				self.0.base.hwnd_ref()
			}
		}
	};
}

/// Implements hwnd() method to control.
macro_rules! pub_fn_hwnd {
	() => {
		/// Returns the underlying handle for this control.
		///
		/// **Note:** the handle is initially null, receiving an actual value
		/// only after the control is physically created, what usually happens
		/// right before
		/// [`WM_CREATE`](crate::gui::events::WindowEvents::wm_create) or
		/// [`WM_INITDIALOG`](crate::gui::events::WindowEvents::wm_init_dialog)
		/// events.
		pub fn hwnd(&self) -> HWND {
			*self.0.base.hwnd_ref()
		}
	};
}

/// Implements ctrl_id() method to control.
macro_rules! pub_fn_ctrlid {
	() => {
		/// Returns the control ID.
		pub fn ctrl_id(&self) -> u16 {
			match &self.0.opts_id {
				OptsId::Wnd(opts) => opts.ctrl_id,
				OptsId::Dlg(ctrl_id) => *ctrl_id,
			}
		}
	};
}

/// Implements focus() method to control.
macro_rules! pub_fn_focus {
	() => {
		/// Focuses the control by sending a
		/// [`wm::NextDlgCtl`](crate::msg::wm::NextDlgCtl) message.
		pub fn focus(&self) -> WinResult<()> {
			self.hwnd().GetParent()?.SendMessage(crate::msg::wm::NextDlgCtl {
				hwnd_focus: crate::enums::HwndFocus::Hwnd(self.hwnd()),
			});
			Ok(())
		}
	};
}

/// Implements on_subclass() method to control.
macro_rules! pub_fn_onsubclass {
	() => {
		/// Exposes the subclass events. If at least one event exists, the
		/// control will be
		/// [subclassed](https://docs.microsoft.com/en-us/windows/win32/controls/subclassing-overview).
		///
		/// **Note:** Subclassing may impact performance, use with care.
		///
		/// # Panics
		///
		/// Panics if the control or the parent window are already created.
		/// Events must be set before control and parent window creation.
		pub fn on_subclass(&self) -> &crate::gui::events::WindowEvents {
			self.0.base.on_subclass()
		}
	};
}

/// Implements on() method to control.
macro_rules! pub_fn_on {
	($evstruc:ident) => {
		/// Exposes the control events.
		///
		/// These event methods are just proxies to the
		/// [`WindowEvents`](crate::gui::events::WindowEvents) of the parent
		/// window, who is the real responsible for the child event handling.
		///
		/// # Panics
		///
		/// Panics if the control or the parent window are already created.
		/// Events must be set before control and parent window creation.
		pub fn on(&self) -> &$evstruc {
			if !self.0.base.hwnd_ref().is_null() {
				panic!("Cannot add events after the control is created.");
			} else if !self.0.base.parent_base_ref().hwnd_ref().is_null() {
				panic!("Cannot add events after the parent window is created.");
			}
			&self.0.events
		}
	};
}
