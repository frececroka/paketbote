(function () {
    const e = React.createElement;

    class DeleteButton extends React.Component {
        constructor(props) {
            super(props);
            this.state = {clicked: false};
        }

        render() {
            let cssClass = "";
            if (this.state.clicked) {
                cssClass = "delete-confirm";
            }
            return e("button", {
                className: "bt-link " + cssClass,
                onClick: (ev) => this.onClick(ev),
                dangerouslySetInnerHTML: {
                    __html: this.props.content
                }
            });
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
        for (let element of elements) {
            let content = element.getElementsByTagName("button")[0].innerHTML;
            ReactDOM.render(e(DeleteButton, { content }), element);
        }
    });
})();
