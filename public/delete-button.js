(function () {
    const e = React.createElement;

    class DeleteButton extends React.Component {
        constructor(props) {
            super(props);
            this.state = {clicked: false};
        }

        render() {
            let cssClass = "";
            let text = this.props.text;
            if (this.state.clicked) {
                cssClass = "delete-confirm";
                text = "[sure?]";
            }
            let style = {};
            if (this.state.minWidth) {
                style["minWidth"] = this.state.minWidth;
            }
            let props = {
                className: "bt-link " + cssClass,
                onClick: (ev) => this.onClick(ev),
                style
            };
            return e("button", props, text);
        }

        onClick(ev) {
            if (!this.state.clicked) {
                ev.preventDefault();
                const minWidth = ev.target.clientWidth + "px";
                this.setState({ clicked: true, minWidth });
                window.clearTimeout(this.timeout);
                this.timeout = window.setTimeout(_ => {
                    this.setState({ clicked: false, minWidth: null })
                }, 1000);
            }
        }
    }

    window.addEventListener("DOMContentLoaded", _ => {
        let elements = document.getElementsByClassName("react-delete-button");
        for (element of elements) {
            let text = element.innerText;
            ReactDOM.render(e(DeleteButton, { text }), element);
        }
    });
})();
