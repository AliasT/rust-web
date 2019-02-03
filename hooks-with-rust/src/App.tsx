import React, { Component, FormEvent } from "react"
import { InputGroup, Button, FormGroup } from "@blueprintjs/core"
import styles from "./App.module.css"
import axios from "axios"

class App extends Component {
  onSubmit = async (event: FormEvent) => {
    event.preventDefault()
    const data = await axios.post("/signup", {
      username: "3",
      password: "3"
    })
    console.log(data)
  }

  render() {
    return (
      <div className="App">
        <form onSubmit={this.onSubmit} className={styles.signupForm}>
          <FormGroup inline>
            <InputGroup leftIcon="user" large />
          </FormGroup>
          <FormGroup inline>
            <InputGroup leftIcon="lock" large />
          </FormGroup>
          <FormGroup inline>
            <Button large type="submit">
              提交
            </Button>
          </FormGroup>
        </form>
      </div>
    )
  }
}

export default App
