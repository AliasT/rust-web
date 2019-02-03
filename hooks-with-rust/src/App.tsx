import React, { Component } from "react"
import { InputGroup, Button, FormGroup } from "@blueprintjs/core"
import styles from "./App.module.css"

class App extends Component {
  onSubmit = () => {}
  render() {
    return (
      <div className="App">
        <form onSubmit={this.onSubmit} className={styles.signupForm}>
          <FormGroup inline>
            <InputGroup leftIcon="user" />
          </FormGroup>
          <FormGroup inline>
            <InputGroup leftIcon="lock" />
          </FormGroup>
          <FormGroup inline>
            <Button>提交</Button>
          </FormGroup>
        </form>
      </div>
    )
  }
}

export default App
