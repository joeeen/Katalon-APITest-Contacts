import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent as HttpTextBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable as GlobalVariable

String randomEmail = CustomKeywords.'helper.random.email'()

String randomPassword = CustomKeywords.'helper.random.password'(7)

body = WS.sendRequestAndVerify(findTestObject('Users/Add user', [('email') : randomEmail, ('password') : randomPassword]))

def slurper = new groovy.json.JsonSlurper()

def response = slurper.parseText(body.getResponseBodyContent())

HttpTextBodyContent request = body.getBodyContent()

def token = response.token

GlobalVariable.token = token

println(response)

println(request.getText())

