(function () {
    const e = React.createElement;

    class DeleteButton extends React.Component {
        constructor(props) {
            super(props);
            this.state = {clicked: false};
        }

        render() {
            let cssClass = "";
            let text = "[delete]";
            if (this.state.clicked) {
                cssClass = "delete-confirm";
                text = "[sure?]";
            }
            let props = {
                className: "bt-link " + cssClass,
                onClick: (ev) => this.onClick(ev)
            };
            return e("button", props, text);
        }

        onClick(ev) {
            if (!this.state.clicked) {
                ev.preventDefault();
                this.setState({ clicked: true });
                window.clearTimeout(this.timeout);
                this.timeout = window.setTimeout(_ => this.setState({ clicked: false }), 1000);
            }
        }
    }

    window.addEventListener("DOMContentLoaded", _ => {
        let elements = document.getElementsByClassName("react-delete-button");
        for (element of elements) {
            ReactDOM.render(e(DeleteButton), element);
        }
    });
})();
